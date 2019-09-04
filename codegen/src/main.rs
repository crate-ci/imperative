use std::io::Write;
use std::iter::FromIterator;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Codegen {
    #[structopt(short("-o"), long, parse(from_os_str))]
    output: std::path::PathBuf,

    #[structopt(long)]
    check: bool,
}

impl Codegen {
    fn write(&self, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        if self.check {
            let content = String::from_iter(normalize_line_endings::normalized(content.chars()));

            let actual = std::fs::read_to_string(&self.output)?;
            let actual = String::from_iter(normalize_line_endings::normalized(actual.chars()));

            let changeset = difference::Changeset::new(&actual, &content, "\n");
            if changeset.distance != 0 {
                eprintln!("{}", changeset);
                return Err(Box::new(CodegenError));
            } else {
                println!("Success");
            }
        } else {
            let mut file = std::io::BufWriter::new(std::fs::File::create(&self.output)?);
            write!(file, "{}", content)?;
        }

        Ok(())
    }
}

#[derive(Copy, Clone, Debug, derive_more::Display)]
#[display(fmt = "Code-gen failed")]
struct CodegenError;

impl std::error::Error for CodegenError {}

pub const VERBS: &str = include_str!("../data/imperatives.txt");
pub const BLACKLIST: &str = include_str!("../data/imperatives_blacklist.txt");

fn parse_wordlist(raw: &str) -> impl Iterator<Item = &str> {
    raw.lines()
        .map(|s| s.splitn(2, "#").next().expect("always at least one").trim())
        .filter(|s| !s.is_empty())
}

fn generate<W: std::io::Write>(file: &mut W) {
    writeln!(
        file,
        "// This file is code-genned by {}",
        env!("CARGO_PKG_NAME")
    )
    .unwrap();
    writeln!(file, "pub(crate) use hack::*;").unwrap();
    writeln!(file, "").unwrap();
    writeln!(file, "#[rustfmt::skip]").unwrap();
    writeln!(file, "mod hack {{").unwrap();

    let en_stemmer = rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::English);
    let words: multimap::MultiMap<_, _> = parse_wordlist(VERBS)
        .map(|s| (en_stemmer.stem(s).into_owned(), s))
        .collect();

    let mut sorted_words: Vec<_> = words.iter_all().collect();
    sorted_words.sort();
    let sorted_words = sorted_words;
    for (stem, words) in sorted_words {
        let mut words = words.clone();
        words.sort();
        let words = words;
        write!(
            file,
            "pub(crate) static r#{}: phf::Set<&'static str> = ",
            stem,
        )
        .unwrap();
        let mut builder = phf_codegen::Set::new();
        for word in words {
            builder.entry(word);
        }
        builder.build(file).unwrap();
        write!(file, ";\n").unwrap();
    }

    let mut stems: Vec<_> = words.keys().collect();
    stems.sort();
    let stems = stems;
    write!(
        file,
        "pub(crate) static IMPERATIVES: phf::Map<&'static str, &phf::Set<&'static str>> = "
    )
    .unwrap();
    let mut builder = phf_codegen::Map::new();
    for stem in stems {
        let value = format!("&r#{}", stem);
        builder.entry(stem.as_str(), &value);
    }
    builder.build(file).unwrap();
    write!(file, ";\n").unwrap();

    let mut blacklist: Vec<_> = parse_wordlist(BLACKLIST).collect();
    blacklist.sort();
    let blacklist = blacklist;
    write!(
        file,
        "pub(crate) static BLACKLIST: phf::Set<&'static str> = "
    )
    .unwrap();
    let mut builder = phf_codegen::Set::new();
    for word in blacklist {
        builder.entry(word);
    }
    builder.build(file).unwrap();
    write!(file, ";\n").unwrap();
    writeln!(file, "}}").unwrap();
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Options {
    #[structopt(flatten)]
    codegen: Codegen,
}

fn run() -> Result<i32, Box<dyn std::error::Error>> {
    let options = Options::from_args();

    let mut buffer = vec![];
    generate(&mut buffer);

    let content = String::from_utf8(buffer)?;
    options.codegen.write(&content)?;

    Ok(0)
}

fn main() {
    let code = run().unwrap();
    std::process::exit(code);
}
