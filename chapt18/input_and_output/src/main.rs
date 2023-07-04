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

fn main() {
    let result = test_writer();
    if let Err(err) = result {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
