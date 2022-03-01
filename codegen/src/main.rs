use clap::Parser;

pub const VERBS: &str = include_str!("../data/imperatives.txt");
pub const BLACKLIST: &str = include_str!("../data/imperatives_blacklist.txt");

fn parse_wordlist(raw: &str) -> impl Iterator<Item = &str> {
    raw.lines()
        .map(|s| s.splitn(2, '#').next().expect("always at least one").trim())
        .filter(|s| !s.is_empty())
}

fn generate<W: std::io::Write>(file: &mut W) {
    writeln!(
        file,
        "// This file is code-genned by {}",
        env!("CARGO_PKG_NAME")
    )
    .unwrap();
    writeln!(file).unwrap();

    let en_stemmer = rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::English);
    let words: multimap::MultiMap<_, _> = parse_wordlist(VERBS)
        .map(|s| (en_stemmer.stem(s).into_owned(), s))
        .collect();

    let mut sorted_words: Vec<_> = words.iter_all().collect();
    sorted_words.sort_unstable();
    let sorted_words = sorted_words;
    for (stem, words) in sorted_words {
        let mut words = words.clone();
        words.sort_unstable();
        let words = words;
        writeln!(
            file,
            "pub(crate) static {}_STEM: phf::Set<&'static str> = ",
            stem.to_uppercase(),
        )
        .unwrap();
        let mut builder = phf_codegen::Set::new();
        for word in words {
            builder.entry(word);
        }
        let codegenned = builder.build();
        writeln!(file, "{}", codegenned).unwrap();
        writeln!(file, ";").unwrap();
        writeln!(file).unwrap();
    }

    let mut stems: Vec<_> = words.keys().collect();
    stems.sort_unstable();
    let stems = stems;
    writeln!(
        file,
        "pub(crate) static IMPERATIVES: phf::Map<&'static str, &phf::Set<&'static str>> = "
    )
    .unwrap();
    let mut builder = phf_codegen::Map::new();
    for stem in stems {
        let value = format!("&{}_stem", stem).to_uppercase();
        builder.entry(stem.as_str(), &value);
    }
    let codegenned = builder.build();
    writeln!(file, "{}", codegenned).unwrap();
    writeln!(file, ";").unwrap();

    let mut blacklist: Vec<_> = parse_wordlist(BLACKLIST).collect();
    blacklist.sort_unstable();
    let blacklist = blacklist;
    writeln!(
        file,
        "pub(crate) static BLACKLIST: phf::Set<&'static str> = "
    )
    .unwrap();
    let mut builder = phf_codegen::Set::new();
    for word in blacklist {
        builder.entry(word);
    }
    let codegenned = builder.build();
    writeln!(file, "{}", codegenned).unwrap();
    writeln!(file, ";").unwrap();
}

#[derive(Debug, Parser)]
#[clap(rename_all = "kebab-case")]
struct Options {
    #[clap(flatten)]
    codegen: codegenrs::CodeGenArgs,
    #[clap(flatten)]
    rustmft: codegenrs::RustfmtArgs,
}

fn run() -> Result<i32, Box<dyn std::error::Error>> {
    let options = Options::parse();

    let mut content = vec![];
    generate(&mut content);

    let content = String::from_utf8(content)?;
    let content = options.rustmft.reformat(&content)?;
    options.codegen.write_str(&content)?;

    Ok(0)
}

fn main() {
    let code = run().unwrap();
    std::process::exit(code);
}
