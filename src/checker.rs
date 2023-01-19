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
        if set_contains(&crate::wordlist_codegen::BLACKLIST, word) {
            return Some(false);
        }

        let stem = self.en_stemmer.stem(word);
        let imperative_forms = map_lookup(&crate::wordlist_codegen::IMPERATIVES, stem.as_ref())?;
        Some(set_contains(imperative_forms, word))
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

fn map_lookup<V: Clone>(map: &'static phf::Map<&'static str, V>, key: &str) -> Option<V> {
    // This transmute should be safe as `get` will not store the reference with
    // the expanded lifetime. This is due to `Borrow` being overly strict and
    // can't have an impl for `&'static str` to `Borrow<&'a str>`.
    //
    //
    // See https://github.com/rust-lang/rust/issues/28853#issuecomment-158735548
    unsafe {
        let key = ::std::mem::transmute::<_, &'static str>(key);
        map.get(key).cloned()
    }
}

fn set_contains(set: &'static phf::Set<&'static str>, key: &str) -> bool {
    // This transmute should be safe as `get` will not store the reference with
    // the expanded lifetime. This is due to `Borrow` being overly strict and
    // can't have an impl for `&'static str` to `Borrow<&'a str>`.
    //
    //
    // See https://github.com/rust-lang/rust/issues/28853#issuecomment-158735548
    unsafe {
        let key = ::std::mem::transmute::<_, &'static str>(key);
        set.contains(key)
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
        ];
        for (word, expected) in cases.iter() {
            println!("Checking {}", word);
            assert_eq!(mood.is_imperative(word), expected.clone());
        }
    }
}
