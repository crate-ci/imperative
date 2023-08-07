pub struct Mood {
    en_stemmer: rust_stemmers::Stemmer,
}

impl Mood {
    pub fn new() -> Self {
        Self {
            en_stemmer: rust_stemmers::Stemmer::create(rust_stemmers::Algorithm::English),
        }
    }

    /// **Note:** that the input is expected to be all lowercase (if that is applicable).
    pub fn is_imperative(&self, word: &str) -> Option<bool> {
        if crate::wordlist_codegen::BLACKLIST.contains(word) {
            return Some(false);
        }

        let stem = match word {
            "added" => "add".into(),
            _ => self.en_stemmer.stem(word),
        };
        let imperative_forms = crate::wordlist_codegen::IMPERATIVES.get(stem.as_ref())?;
        Some(imperative_forms.contains(word))
    }
}

impl Clone for Mood {
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl Default for Mood {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_imperative() {
        let mood = Mood::new();
        let cases = &[
            ("runs", Some(false)),
            ("run", Some(true)),
            ("returns", Some(false)),
            ("return", Some(true)),
            ("constructor", Some(false)),
            ("adds", Some(false)),
            ("add", Some(true)),
            ("added", Some(false)),
        ];
        for (word, expected) in cases.iter() {
            println!("Checking {}", word);
            assert_eq!(mood.is_imperative(word), expected.clone());
        }
    }
}
