use async_std::io::prelude::*;


pub async fn test_async_blocks() -> Result<_, _> {
    let input = async_std::io::stdin();
    let future = async {
        let mut line = String::new();

        // This returns `std::io::Result<usize>`.
        input.read_line(&mut line).await?;

        println!("Read line: {}", line);

        Ok::<(), std::io::Error>(())
    };
    //Ok(future)
}
fn main() {
    let result = async_std::task::block_on(test_async_blocks());
    /*
    match result {
        Ok(v) => println!("{v:?}"),
        Err(e) => println!("{e:?}"),
    }
    */

}
