fn main() {
    assert_eq!("カニ".chars().next(), Some('カ'));

    // classifying characters
    // char type
    assert!('4'.is_numeric());
    assert!('ᛮ'.is_numeric());
    assert!('⑧'.is_numeric());
    assert!('q'.is_alphabetic());
    assert!('七'.is_alphabetic());
    assert!('9'.is_alphanumeric());
    assert!('饂'.is_alphanumeric());
    assert!(!'*'.is_alphanumeric());
    assert!(' '.is_whitespace());
    assert!('\n'.is_whitespace());
    assert!('\u{A0}'.is_whitespace());
    assert!('\n'.is_control());
    assert!('\u{85}'.is_control());
    // ASCII char
    assert!('n'.is_ascii());
    assert!(!'ñ'.is_ascii());
    assert!('n'.is_ascii_alphabetic());
    assert!(!'1'.is_ascii_alphabetic());
    assert!(!'ñ'.is_ascii_alphabetic());
    assert!('8'.is_ascii_digit());
    assert!(!'-'.is_ascii_digit());
    assert!(!'⑧'.is_ascii_digit());
    assert!('q'.is_ascii_alphanumeric());
    assert!('0'.is_ascii_alphanumeric());
    assert!('\n'.is_ascii_control());
    assert!('\x7f'.is_ascii_control());
    assert!('Q'.is_ascii_graphic());
    assert!('~'.is_ascii_graphic());
    assert!(!' '.is_ascii_graphic());
    assert!('z'.is_ascii_lowercase());
    assert!('Z'.is_ascii_uppercase());
    assert!(' '.is_ascii_whitespace());
    assert!('\n'.is_ascii_whitespace());
    assert!(!'\u{A0}'.is_ascii_whitespace());
    // u8 byte type
    assert!(32u8.is_ascii_whitespace());
    assert!(b'9'.is_ascii_digit());

    let line_tab = '\u{000b}'; // 'line tab', AKA 'vertical tab'
    assert_eq!(line_tab.is_whitespace(), true);
    assert_eq!(line_tab.is_ascii_whitespace(), false);

    // handling digits
    assert_eq!('F'.to_digit(16), Some(15));
    assert_eq!(std::char::from_digit(15, 16), Some('f'));
    assert!(char::is_digit('f', 16));
    assert!('f'.to_digit(16) != None);

    // case conversion for characters
    let mut upper = 's'.to_uppercase();
    assert_eq!(upper.next(), Some('S'));
    assert_eq!(upper.next(), None);
    // The uppercase from of the German letter "sharp S" is "SS":
    let mut upper = 'ß'.to_uppercase();
    assert_eq!(upper.next(), Some('S'));
    assert_eq!(upper.next(), Some('S'));
    assert_eq!(upper.next(), None);

    // Unicode says to lowercase Turkish dotted capital 'İ' to 'i'
    // followed by `'\u{307}'`, COMBINING DOT ABOVE, so that a 
    // subsequent conversion back to uppercase preserves the dot.
    let ch = 'İ'; // `'\u{130}'`
    let mut lower = ch.to_lowercase();
    assert_eq!(lower.next(), Some('i'));
    assert_eq!(lower.next(), Some('\u{307}'));
    assert_eq!(lower.next(), None);

    // Conversions to and from Integers
    assert_eq!('B' as u32, 66);
    assert_eq!('饂' as u8, 66); // upper bits trucated
    assert_eq!('二' as i8, -116); // same
    assert_eq!(char::from(66), 'B');
    assert_eq!(std::char::from_u32(0x9942), Some('饂'));
    assert_eq!(std::char::from_u32(0xd800), None); // reserved for UTF-16

}
