use std::error::Error;
use std::io::prelude::*;
use std::str::FromStr;


fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    // sums numbers read from its standard input
    let sum = stdin.lock()
        .lines()
        .try_fold(0, |sum, line| -> Result<u64, Box<dyn Error>> {
            Ok(sum + u64::from_str(&line?.trim())?)
        })?;
    println!("sum: {}", sum);
    Ok(())
}

/*
fn all<P>(&mut self, mut predicate: P) -> bool
    where P: FnMut(Self::Item) -> bool,
          Self: Sized
{ 
    self.try_fold((), |_, item| {
        if predicate(item) { Some(()) } else { None }
    }).is_some()

}
*/

