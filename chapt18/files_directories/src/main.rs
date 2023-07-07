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

    //use std::path::Path;
    let home_dir = Path::new("home/fwolfe");

    assert_eq!(Path::new("/home/fwolfe/program.txt").parent(),
        Some(Path::new("/home/fwolfe")));
    //use std::ffi::OsStr;
    assert_eq!(Path::new("/home/fwolfe/program.txt").file_name(),
        Some(OsStr::new("program.txt")));
    
    assert_eq!(home_dir.is_absolute(), false);
    let path1 = Path::new("/usr/share/dict");
    assert_eq!(path1.join("words"),
        Path::new("/usr/share/dict/words"));
    let abs_path = std::env::current_dir().unwrap().join(Path::new("/bin"));
    println!("abs_path:{}", abs_path.display());
    let file = Path::new("/home/jimb/calendars/calendar-18x18.pdf");
    assert_eq!(file.ancestors().collect::<Vec<_>>(),
        vec![Path::new("/home/jimb/calendars/calendar-18x18.pdf"),
            Path::new("/home/jimb/calendars"),
            Path::new("/home/jimb"),
            Path::new("/home"),
            Path::new("/")]);
    if let Some(file_str) = path1.to_str() {
        println!("{}", file_str);
    } // ...otherwise skip this weirdly named file

    println!("Download found. You put it in: {}", home_dir.display());

    let path = Path::new("/tmp/foo/bar.txt");

    for component in path.components() {
        println!("{component:?}");
    }
}
