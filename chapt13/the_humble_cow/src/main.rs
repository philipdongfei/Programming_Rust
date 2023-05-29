use std::path::PathBuf;
use std::borrow::Cow;
use std::fs::File;
use std::io::{Error, ErrorKind};

fn describe(errkind: &ErrorKind) -> Cow<'static, str> {
    match *errkind {
        ErrorKind::OutOfMemory => "out of memory".into(),
        //ErrorKind::StackOverflow => "stack overflow".into(),
        //ErrorKind::MachineOnFire => "machine on fire".into(),
        //ErrorKind::Unfathomable => "machine bewildered".into(),
        ErrorKind::NotFound => {
            format!("file not found: {errkind}").into()
        },
        _ => "other".into() 

    }
}

fn remove_whitespaces(s: &str) -> Cow<str> {
    if s.contains(' ') {
        Cow::Owned(s.to_string().replace(' ', ""))
    } else {
        Cow::Borrowed(s)
    }
}

fn main() {
    let value = remove_whitespaces("Hello world!");
    println!("{}", value);
    

    // Cow error
    let _file = match File::open("info.txt") {
        Ok(file) => file,
        Err(error) => {
            let mut log: Vec<String> = Vec::new();
            log.push(describe(&error.kind()).into_owned());
            panic!("Disaster has struck: {}", describe(&error.kind()));
        }
    };

}
