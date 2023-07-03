fn main() {
    assert!("th\u{e9}" != "the\u{301}");
    assert!("th\u{e9}" >  "the\u{301}");

    // A Hasher is designed to accumulate the hash of a series of values,
    // so hashing just one is a bit clunky.
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;
    fn hash<T: ?Sized + Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    // These values may change in future Rust releases.
    assert_eq!(hash("th\u{e9}"), 0x53e2d0734eb1dff3);
    assert_eq!(hash("the\u{301}"), 0x90d837f0a0928144);

    // The unicode-normalization Crate
    use unicode_normalization::UnicodeNormalization;

    // No matter what representation the left-hand string uses
    // (you shouldn't be able to tell just by looking),
    // these assertions will hold.
    assert_eq!("Phở".nfd().collect::<String>(), "Pho\u{31b}\u{309}");
    assert_eq!("Phở".nfc().collect::<String>(), "Ph\u{1edf}");

    // The left-hand side here uses the "ffi" ligature character.
    assert_eq!("① Di\u{fb03}culty".nfkc().collect::<String>(), "1 Difficulty");
}

