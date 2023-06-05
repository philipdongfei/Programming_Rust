fn main() {
    //use std::iter::FromIterator;

    let mut outer = "Earth".to_string();
    let inner = String::from_iter(outer.drain(1..4));

    assert_eq!(outer, "Eh");
    assert_eq!(inner, "art");

    let mut word = "Pig".to_string();
    let out = String::from_iter(word.drain(..));
    assert_eq!(out, "Pig".to_string());
    assert_eq!(word, "");
}
