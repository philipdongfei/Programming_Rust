use std::fs;
use std::io;
use std::path::Path;
use std::env;

#[cfg(unix)]
use std::os::unix::fs::symlink;

use std::os::unix::prelude::*; // `prelude` module that can be used to enable all of these extensions at once

/// Stub implementation of `symlink` for platforms that don't provide it.

#[cfg(not(unix))]
fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, _dst: Q)
    -> std::io::Result<()>
{
    Err(io::Error::new(io::ErrorKind::Other,
            format!("can't copy symbolic link! {}",
                src.as_ref().display())))
}

/// Copy the existing directory `src` to the target path `dst`.
fn copy_dir_to(src: &Path, dst: &Path) -> io::Result<()> {
    if !dst.is_dir() {
        fs::create_dir(dst)?;
    }
    for entry_result in src.read_dir()? {
        let entry = entry_result?;
        let file_type = entry.file_type()?;
        copy_to(&entry.path(), &file_type, &dst.join(entry.file_name()))?;
    }
    Ok(())
}

/// Copy whatever is at `src` to the target path `dst`.
fn copy_to(src: &Path, src_type: &fs::FileType, dst: &Path) 
    -> io::Result<()>
{
    if src_type.is_file() {
        fs::copy(src, dst)?;
    } else if src_type.is_dir() {
        copy_dir_to(src, dst)?;
    } else if src_type.is_symlink() {
        let target = src.read_link()?;
        symlink(target, dst)?;

    } else {
        return Err(io::Error::new(io::ErrorKind::Other,
                format!("don't know how to copy: {}",
                    src.display())));
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let result = copy_dir_to(Path::new(&args[0]), Path::new(&args[1]));
    if let Err(err) = result {
        eprintln!("{}", err);
        std::process::exit(1);
    }
    println!("copy dir success!");
}
