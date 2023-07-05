use std::io::BufWriter;
use std::fs::File;
// Readers and Writers
use std::io::{self, Read, Write, ErrorKind};

const DEFAULT_BUF_SIZE: usize = 8 * 1024;

pub fn copy<R: ?Sized, W: ?Sized>(reader: &mut R, writer: &mut W)
    -> io::Result<u64>
    where R: Read, W: Write
{
    let mut buf = [0; DEFAULT_BUF_SIZE];
    let mut written = 0;
    loop {
        let len = match reader.read(&mut buf) {
            Ok(0) => return Ok(written),
            Ok(len) => len,
            Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };
        writer.write_all(&buf[..len])?;
        written += len as u64;
    }
}

fn test_writer() -> io::Result<()> 
{
    writeln!(io::stderr(), "error: world not helloable")?;
    let file = File::create("tmp.txt")?;
    let mut writer = BufWriter::new(file);
    write!(writer, "too much data")?;
    writer.flush().expect_err("it doesn't fit");
    Ok(())
}

fn test_file() -> io::Result<()>
{
    // File
    use std::fs::OpenOptions;
    let mut log = OpenOptions::new()
        .append(true) // if file exists, add to the end
        .open("server.log")?;
    log.write_all(b"test server log");
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true) // fail if file exists
        .open("new_file.txt")?;
    file.write_all(b"test new_file.txt");
    Ok(())

}

fn test_other() -> io::Result<()> {
    use std::process::{Command, Stdio};

    let mut child = 
        Command::new("grep")
        .arg("-e")
        .arg("a.*e.*i.*o.*u")
        .stdin(Stdio::piped())
        .spawn()?;

    let my_words = vec!["a one i o u".to_string(), "two".to_string(), 
    "three".to_string(), "four".to_string(), "five".to_string()];
    let mut to_child = child.stdin.take().unwrap();
    for word in my_words {
        writeln!(to_child, "{}", word)?;
    }
    drop(to_child);  // close grep's stdin, so it will exit
    child.wait()?;
    Ok(())
} 

fn main() {
    let _ = test_other();
    println!("test_file:");
    let result = test_file();
    if let Err(err) = result {
        eprintln!("{}", err);
        std::process::exit(1);
    }
    println!("test_writer:");
    let result = test_writer();
    if let Err(err) = result {
        eprintln!("{}", err);
        std::process::exit(1);
    }

}
