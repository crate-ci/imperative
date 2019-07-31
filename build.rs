use std::io::Write;

pub const VERBS: &str = include_str!("./data/imperatives.txt");
pub const BLACKLIST: &str = include_str!("./data/imperatives_blacklist.txt");

fn parse_wordlist(raw: &str) -> impl Iterator<Item = &str> {
    raw.lines()
        .map(|s| s.splitn(2, "#").next().expect("always at least one").trim())
        .filter(|s| !s.is_empty())
}

fn main() {
    let path = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = std::io::BufWriter::new(std::fs::File::create(&path).unwrap());

    println!("rerun-if-changed=./data/imperatives.txt");
    println!("rerun-if-changed=./data/imperatives_blacklist.txt");

    let en_stemmer = rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::English);
    let words: multimap::MultiMap<_, _> = parse_wordlist(VERBS)
        .map(|s| (en_stemmer.stem(s).into_owned(), s))
        .collect();

    for (stem, words) in words.iter_all() {
        write!(
            &mut file,
            "pub(crate) static r#{}: phf::Set<&'static str> = ",
            stem,
        )
        .unwrap();
        let mut builder = phf_codegen::Set::new();
        for word in words {
            builder.entry(*word);
        }
        builder.build(&mut file).unwrap();
        write!(&mut file, ";\n").unwrap();
    }

    write!(
        &mut file,
        "pub(crate) static IMPERATIVES: phf::Map<&'static str, &phf::Set<&'static str>> = "
    )
    .unwrap();
    let mut builder = phf_codegen::Map::new();
    for stem in words.keys() {
        let value = format!("&r#{}", stem);
        builder.entry(stem.as_str(), &value);
    }
    builder.build(&mut file).unwrap();
    write!(&mut file, ";\n").unwrap();

    write!(
        &mut file,
        "pub(crate) static BLACKLIST: phf::Set<&'static str> = "
    )
    .unwrap();
    let mut builder = phf_codegen::Set::new();
    for word in parse_wordlist(BLACKLIST) {
        builder.entry(word);
    }
    builder.build(&mut file).unwrap();
    write!(&mut file, ";\n").unwrap();
}
