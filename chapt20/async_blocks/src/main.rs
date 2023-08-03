use async_std::io::prelude::*;
use async_std::net;

async fn cheapo_request(host: &str, port: u16, path: &str)
                    -> std::io::Result<String>
{
    let mut socket = net::TcpStream::connect((host, port)).await?;

    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);
    socket.write_all(request.as_bytes()).await?;
    socket.shutdown(net::Shutdown::Write)?;

    let mut response = String::new();
    socket.read_to_string(&mut response).await?;

    Ok(response)
}

async fn cheapo_owning_request(host: String, port: u16, path: String)
    -> std::io::Result<String> {
        cheapo_request(&host, port, &path).await
}

async fn many_requests(requests: Vec<(String, u16, String)>)
                    -> Vec<std::io::Result<String>>
{
    use async_std::task;

    let mut handles = vec![];
    // async blocks
    for (host, port, path) in requests {
        handles.push(task::spawn_local(async move {
            cheapo_request(&host, port, &path).await
        }));
    }
    /*
     // the cheapo_owning_request wrapper function
    for (host, port, path) in requests {
        handles.push(task::spawn_local(cheapo_owning_request(host, port, path)));
    }
    */

    let mut results = vec![];
    for handle in handles {
        results.push(handle.await);
    }

    results
}

fn test_async_block_return() 
{
    eprintln!("test async block begin:");
    let input = async_std::io::stdin();
    let future = async {
        let mut line = String::new();

        // This returns `std::io::Result<usize>`.
        input.read_line(&mut line).await?;

        println!("Read line: {}", line);

        Ok::<(), std::io::Error>(())
    };
    eprintln!("test async block end:");

}


fn main() {
     
    let requests = vec![
        ("example.com".to_string(),      80, "/".to_string()),
        ("www.red-bean.com".to_string(), 80, "/".to_string()),
        ("en.wikipedia.org".to_string(), 80, "/".to_string()),
    ];

    let results = async_std::task::block_on(many_requests(requests));
    for result in results {
        match result {
            Ok(response) => println!("{}", response),
            Err(err) => eprintln!("error: {}", err),
        }
    }

    // Rust can't tell what the return type of the async block should be.
    // For now, you can work around the problem by spelling out the type of the block's final Ok
    let input = async_std::io::stdin();
    let future = async {
        let mut line = String::new();

        // This returns `std::io::Result<usize>`.
        input.read_line(&mut line).await?;

        println!("Read line: {}", line);

        Ok::<(), std::io::Error>(())
    };
    //test_async_block_return();
    /*
    let result = async_std::task::block_on(test_async_block_return());
    match result {
        Ok(usize) => println!("input: {}", usize),
        Err(err) => eprintln!("error: {}", err),

    };
    */
}
