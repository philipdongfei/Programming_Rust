fn main() {
    let message = "To: jimb\r\n\
    From: id\r\n\
    \r\n\
    Oooooh, donuts!!\r\n";

    let mut lines = message.lines();

    println!("Headers:");
    for header in lines.by_ref().take_while(|l| !l.is_empty()) {
        println!("{}", header);
    }

    println!("\nBody:");
    for body in lines {
        println!("{}", body);
    }

    // basic usage
    let mut words = ["hello", "world", "of", "Rust"].into_iter();

    // Take the first two words.
    let hello_world: Vec<_> = words.by_ref().take(2).collect();
    assert_eq!(hello_world, vec!["hello", "world"]);

    // Collect the rest of the words.
    // We can only do this because we used `by_ref` earlier.
    let of_rust: Vec<_> = words.collect();
    assert_eq!(of_rust, vec!["of", "Rust"]);
}
