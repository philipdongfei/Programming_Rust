fn main() {
    // take_while
    let message = "To: jimb\r\n\
        From: superego <editor@oreilly.com>\r\n\
        \r\n\
        Did you get any writing done today?\r\n\
        When will you stop wasting time plotting fractals?\r\n";
    for header in message.lines().take_while(|l| !l.is_empty()) {
        println!("{}", header);
    }

    // take
    let a = [1, 2, 3];

    let mut iter = a.iter().take(2);

    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), None);

    let mut iter = (0..).take(3);
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), None);

}
