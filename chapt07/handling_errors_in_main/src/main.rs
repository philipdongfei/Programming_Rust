fn main() {
    // the simplest way
    calculate_tides().expect("error"); // the buck stops here
    // have more complex error types or want to include more details in your message.
    if let Err(err) = calculate_tides() {
        print_error(&err);
        std::process::exit(1);
    }
}
