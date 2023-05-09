use std::io::Write;



fn main() {
    let mut buf: Vec<u8> = vec![];
    //buf.write_all(b"hello")?;
    buf.write_all(b"hello").expect("error");

    if let Err(err) = buf.write_all(b" world") {
        //print_error(&err);
        std::process::exit(1);
    }
    print!("{:?}\n", buf);
}
