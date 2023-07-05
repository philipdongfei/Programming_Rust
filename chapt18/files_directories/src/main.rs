use std::path::Path;
use std::io::{self, Write};
use std::ffi::OsStr;
use std::fs::File;

fn swizzle_file<P>(path_arg: P) -> io::Result<()> 
    where P: AsRef<Path>
{
    let path = path_arg.as_ref();
    let mut buffer = File::create(path)?;
    buffer.write(b"test OsStr")?;

    Ok(())
}
fn main() {
    let os_str = OsStr::new("foo");
    let result = swizzle_file(&os_str);
    if let Err(err) = result {
        eprintln!("{}", err);
        std::process::exit(1);
    }        

}
