fn main() {
    // creating string values 
    let spacey = "man hat tan";
    let spaceless: String = 
        spacey.chars().filter(|c| !c.is_whitespace()).collect();
    assert_eq!(spaceless, "manhattan");

    // simple inspection
    let full = "bookkeeping";
    assert_eq!(&full[..4], "book");
    assert_eq!(&full[5..], "eeping");
    assert_eq!(&full[2..4], "ok");
    assert_eq!(full[..].len(), 11);
    assert_eq!(full[5..].contains("boo"), false);
    let parenthesized = "Rust (饂)";
    assert_eq!(parenthesized[6..].chars().next(), Some('饂'));

    // appending and inserting text
    use std::fmt::Write;

    let mut letter = String::new();
    writeln!(letter, "Whose {} these are I think I know", "rutabagas").unwrap();
    writeln!(letter, "His house is in the village though;").unwrap();
    assert_eq!(letter, "Whose rutabagas these are I think I know\n\
His house is in the village though;\n");

    let left = "partners".to_string();
    let mut right = "crime".to_string();
    assert_eq!(left + " in " + &right, "partners in crime");

    right += " doesn't pay";
    assert_eq!(right, "crime doesn't pay");

    let string = "string".to_string();
    let parenthetical = "(".to_string() + &string + ")";
    assert_eq!(parenthetical, "(string)");

    // removing and replacing text
    let mut choco = "chocolate".to_string();
    assert_eq!(choco.drain(3..6).collect::<String>(), "col");
    assert_eq!(choco, "choate");

    let mut winston = "Churchill".to_string();
    winston.drain(2..6);
    assert_eq!(winston, "Chill");

    let mut beverage = "a piña colada".to_string();
    beverage.replace_range(2..7, "kahlua"); // 'ñ' is two bytes!
    assert_eq!(beverage, "a kahlua colada");

    // conventions for searching and iterating
    // char_indices basic usage
    let word = "goodbye";

    let count = word.char_indices().count();
    assert_eq!(7, count);

    let mut char_indices = word.char_indices();

    assert_eq!(Some((0, 'g')), char_indices.next());
    assert_eq!(Some((1, 'o')), char_indices.next());
    assert_eq!(Some((2, 'o')), char_indices.next());
    assert_eq!(Some((3, 'd')), char_indices.next());
    assert_eq!(Some((4, 'b')), char_indices.next());
    assert_eq!(Some((5, 'y')), char_indices.next());
    assert_eq!(Some((6, 'e')), char_indices.next());
    assert_eq!(None, char_indices.next());

    // patterns for searching text
    let haystack = "One fine day, in the middle of the night";

    assert_eq!(haystack.find(','), Some(12));
    assert_eq!(haystack.find("night"), Some(35));
    assert_eq!(haystack.find(char::is_whitespace), Some(3));
    assert_eq!("## Elephants"
                .trim_start_matches(|ch: char| ch == '#' || ch.is_whitespace()), "Elephants");

        let code = "\t      function noodle() { ";
        // pub fn trim_start_matches<'a, P>(&'a self, pat: P) -> &'a str where P: Pattern<'a>, 
        // fn as_ref(&self) -> &str
        assert_eq!(code.trim_start_matches([' ', '\t'].as_ref()),
                    "function noodle() { ");
        // Shorter equivalent: &[' ', '\t'][..]
        assert_eq!(code.trim_start_matches(&[' ', '\t'][..]), "function noodle() { ");

        // searching and replacing
        assert!("2017".starts_with(char::is_numeric));
        assert!("2abc".starts_with(char::is_numeric));
        assert!("abc7".ends_with(char::is_numeric));

        let quip = "We also know there are known unknowns";
        assert_eq!(quip.find("know"), Some(8));
        assert_eq!(quip.rfind("know"), Some(31));
        assert_eq!(quip.find("ya know"), None);
        assert_eq!(quip.rfind(char::is_uppercase), Some(0));
        
        assert_eq!("The only thing we have to fear is fear itself"
            .replace("fear", "spin"),
            "The only thing we have to spin is spin itself");
        assert_eq!("`Borrow` and `BorrowMut`"
            .replace(|ch:char| !ch.is_alphanumeric(), ""),
            "BorrowandBorrowMut");
        assert_eq!("cabababababbage"
            .replace("aba", "***"),
            "c***b***babbage");
        let s = "this is old";
        assert_eq!(s, s.replace("cookie monster", "little lamb"));

        let s = "foo foo 123 foo";
        assert_eq!("new new 123 foo", s.replacen("foo", "new", 2));
        assert_eq!("faa fao 123 foo", s.replacen('o', "a", 3));
        assert_eq!("foo foo new23 foo", s.replacen(char::is_numeric, "new", 1));

        // iterating over text
        assert_eq!("élan".char_indices().collect::<Vec<_>>(),
            vec![(0, 'é'), // has a two-byte UTF-8 encoding
                 (2, 'l'),
                 (3, 'a'),
                 (4, 'n')]);
        assert_eq!("élan".bytes().collect::<Vec<_>>(),
                vec![195, 169, b'l', b'a', b'n']);
        // The ':' characters are separators here. Note the final "".
        assert_eq!("jimb:1000:Jim Blandy:".split(':').collect::<Vec<_>>(),
            vec!["jimb", "1000", "Jim Blandy", ""]);
        // The '|n' characters are terminators here.
        assert_eq!("127.0.0.1  localhost\n\
                    127.0.0.1  www.reddit.com\n"
                    .split_terminator('\n').collect::<Vec<_>>(),
                    vec!["127.0.0.1  localhost",
                         "127.0.0.1  www.reddit.com"]);
                    // Note, no final ""!
        let poem = "This  is  just   to say\n\
                    I have eaten\n\
                    the plums\n\
                    again\n";
        assert_eq!(poem.split_whitespace().collect::<Vec<_>>(),
                   vec!["This", "is", "just", "to", "say",
                   "I", "have", "eaten", "the", "plums",
                   "again"]);

        let v: Vec<_> = "abcXXXabcYYYabc".match_indices("abc").collect();
        assert_eq!(v, [(0, "abc"), (6, "abc"), (12, "abc")]);
        let v: Vec<_> = "1abcabc2".match_indices("abc").collect();
        assert_eq!(v, [(1, "abc"), (4, "abc")]);
        let v: Vec<_> = "ababa".match_indices("aba").collect();
        assert_eq!(v, [(0, "aba")]); // only the first `aba`

        //trimming
        assert_eq!("\t*.rs  ".trim(), "*.rs");
        assert_eq!("\t*.rs  ".trim_start(), "*.rs  ");
        assert_eq!("\t*.rs  ".trim_end(), "\t*.rs");
        assert_eq!("001990".trim_start_matches('0'), "1990");

        // parsing other types from strings
        use std::str::FromStr;

        assert_eq!(usize::from_str("3628800"), Ok(3628800));
        assert_eq!(f64::from_str("128.5625"), Ok(128.5625));
        assert_eq!(bool::from_str("true"), Ok(true));

        assert!(f64::from_str("not a float at all").is_err());
        assert!(bool::from_str("TRUE").is_err());

        assert_eq!(char::from_str("é"), Ok('é'));
        assert!(char::from_str("abcdefg").is_err());

        use std::net::IpAddr;

        let address = IpAddr::from_str("fe80::0000:3ea9:f4ff:fe34:7a50").unwrap();
        assert_eq!(address,
            IpAddr::from([0xfe80, 0, 0, 0, 0x3ea9, 0xf4ff, 0xfe34, 0x7a50]));
        let address = "fe80::0000:3ea9:f4ff:fe34:7a50".parse::<IpAddr>().unwrap();

        // converting other types to strings
        assert_eq!(format!("{}, wow", "doge"), "doge, wow");
        assert_eq!(format!("{}", true), "true");
        assert_eq!(format!("({:.3}, {:.3})", 0.5, f64::sqrt(3.0)/2.0),
                "(0.500, 0.866)");

        // Using `address` from above.
        let formatted_addr: String = format!("{}", address);
        assert_eq!(formatted_addr, "fe80::3ea9:f4ff:fe34:7a50");
        // continued from above.
        assert_eq!(address.to_string(), "fe80::3ea9:f4ff:fe34:7a50");
        // continued from above.
        let addresses = vec![address,
                        IpAddr::from_str("192.168.0.1").unwrap()];
        assert_eq!(format!("{:?}", addresses),
            "[fe80::3ea9:f4ff:fe34:7a50, 192.168.0.1]");

        // accessing text as utf-8
        let bytes = "bors".as_bytes();
        assert_eq!(b"bors", bytes);

        let s = String::from("hello");
        let bytes = s.into_bytes();
        assert_eq!(&[104, 101, 108, 108, 111][..], &bytes[..]);


        // producing text from utf-8 Data
        use std::str;

        // some bytes, in a vector
        let sparkle_heart = vec![240, 159, 146, 150];

        // We know these bytes are valid, so just use `unwrap()`.
        let sparkle_heart = str::from_utf8(&sparkle_heart).unwrap();

        assert_eq!("💖", sparkle_heart);

        let good_utf8: Vec<u8> = vec![0xe9, 0x8c, 0x86];
        assert_eq!(String::from_utf8(good_utf8).ok(), Some("錆".to_string()));

        let bad_utf8: Vec<u8> = vec![0x9f, 0xf0, 0xa6, 0x80];
        let result = String::from_utf8(bad_utf8);
        assert!(result.is_err());
        // Since String::from_utf8 failed, it didn't consume the original
        // vector, and the error value hands it back to us unharmed.
        assert_eq!(result.unwrap_err().into_bytes(),
            vec![0x9f, 0xf0, 0xa6, 0x80]);

        // some invalid bytes
        let input = b"Hello \xF0\x90\x80World";
        let output = String::from_utf8_lossy(input);

        assert_eq!("Hello �World", output);

        // some bytes, in a vector
        let sparkle_heart = vec![240, 159, 146, 150];

        let sparkle_heart = unsafe {
            String::from_utf8_unchecked(sparkle_heart)
        };

        assert_eq!("💖", sparkle_heart);



}
