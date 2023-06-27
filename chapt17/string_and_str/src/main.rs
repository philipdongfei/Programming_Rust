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

}
