use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref SEMVER: Regex
        = Regex::new(r"(\d+)\.(\d+)\.(\d+)(-[-.[:alnum:]]*)?")
            .expect("error parsing regex");

}

fn main() {
    test_regex();
    
}

fn test_regex() {
    // basic regex use
    // A semver version number, like 0.2.1.
    // May contain a pre-release version suffix, like 0.2.1-alpha.
    // (No build metadata suffix, for brevity.)
    //
    // Note use of r"..." raw string syntax, to avoid backslash blizzard.
    let semver = Regex::new(r"(\d+)\.(\d+)\.(\d+)(-[-.[:alnum:]]*)?").unwrap();

    // Simple search, with a Boolean result.
    let haystack = r#"regex = "0.2.5""#;
    assert!(semver.is_match(haystack));
    
    // You can retrieve capture groups:
    let captures = semver.captures(haystack)
        .ok_or("semver regex should have matched").unwrap();
    assert_eq!(&captures[0], "0.2.5");
    assert_eq!(&captures[1], "0");
    assert_eq!(&captures[2], "2");
    assert_eq!(&captures[3], "5");

    assert_eq!(captures.get(4), None);
    assert_eq!(captures.get(3).unwrap().start(), 13);
    assert_eq!(captures.get(3).unwrap().end(), 14);
    assert_eq!(captures.get(3).unwrap().as_str(), "5");

    let haystack = "In the beginning, there was 1.0.0. \
                    For a while, we used 1.0.1-beta, \
                    but in the end, we settled on 1.2.4.";
    let matches: Vec<&str> = semver.find_iter(haystack)
        .map(|match_| match_.as_str())
        .collect();
    assert_eq!(matches, vec!["1.0.0", "1.0.1-beta", "1.2.4"]);

    // building regex values lazily
    use std::io::BufRead;

    let stdin = std::io::stdin();
    for line_result in stdin.lock().lines() {
        let line = line_result.unwrap();
        if let Some(match_) = SEMVER.find(&line) {
            println!("{}", match_.as_str());
        }
    }

}
