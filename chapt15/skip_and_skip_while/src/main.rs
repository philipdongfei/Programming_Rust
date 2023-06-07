fn main() {
    // skip_while
    let message = "To: jimb\r\n\
        From: superego <editor@oreilly.com>\r\n\
        \r\n\
        Did you get any writing done today?\r\n\
        When will you stop wasting time plotting fractals?\r\n";

    for body in message.lines()
        .skip_while(|l| !l.is_empty())
            .skip(1) {
        println!("{}", body);
    }

    // skip 
    let a = [1, 2, 3];

    let mut iter = a.iter().skip(2);

    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), None);
}
