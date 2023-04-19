use std::io::{self, BufRead};

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;

/// Read integers from a text file.
/// The file should have one number on each line.
fn read_numbers(file: &mut dyn BufRead) -> GenericResult<Vec<i64>> {
    let mut numbers = vec![];
    for line_result in file.lines() {
        let line = line_result?;  // reading lines can fail
        numbers.push(line.parse()?); // parsing integers can fail
    }
    Ok(numbers)
}

// fn Err in Chapter 13.
fn conversion_error() -> Result<&String, io::Error> {
    let io_error = io::Error::new( // make our own io::Error
        io::ErrorKind::Other, "timed out");
    return Err(GenericError::from(io_error)); // manually convert to GenericError
}

fn main() {
    // use the generic method error.downcast_ref::<ErrorType>(). It borrows a reference to
    // the error, if it happends to be the particular type of error you're looking for 
    loop {
        match compile_project() {
            Ok(()) => return Ok(()),
            Err(err) => {
                if let Some(mse) = err.downcast_ref::<MissingSemicolonError>() {
                    insert_semicolon_in_source_code(mse.file(), mse.line())?;
                    continue; // try again!
                }
                return Err(err);
            }
        }
    }
}

