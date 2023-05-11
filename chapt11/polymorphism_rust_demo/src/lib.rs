use std::io::Write;

// use Trait
//fn say_hello(out: &mut dyn Write) -> std::io::Result<()> { // plain function
fn say_hello<W: Write>(out: &mut W) -> std::io::Result<()> { // generic function
    out.write_all(b"hello world\n")?;
    out.flush()
}

#[test]
fn test_say_hello() -> std::io::Result<()> {
    use std::fs::File;
    let mut local_file = File::create("hello.txt")?;
    say_hello(&mut local_file)?; // works
    //say_hello::<File>(&mut local_file)?; // spell out the type parameters

    let mut bytes = vec![];
    say_hello(&mut bytes)?; // also works
    assert_eq!(bytes, b"hello world\n");
    Ok(()) // phew!

}


/// Given two values, pick whickever one is less.
fn min<T: Ord>(value1: T, value2: T) -> T { // use Generics
    
    if value1 <= value2 {
        value1
    } else {
        value2
    }
}
#[test]
fn test_min() {
    assert_eq!(min(1, 2), 1);
}
