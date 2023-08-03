use async_std::io::prelude::*;
use async_std::net;

async fn cheapo_request(host: &str, port: u16, path: &str)
                    -> std::io::Result<String>
{
    eprintln!("{}, cheapo_request begin !", host);
    let mut socket = net::TcpStream::connect((host, port)).await?;
    eprintln!("{}, net::TcpStream::connect await end!", host);

    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);
    socket.write_all(request.as_bytes()).await?;
    eprintln!("{}, socket write_all await end!", host);
    socket.shutdown(net::Shutdown::Write)?;

    let mut response = String::new();
    socket.read_to_string(&mut response).await?;
    eprintln!("{}, socket read_to_string await end!\n###########", host);

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
    for (host, port, path) in requests {
        // use async_std::task::spawn to spawn a future onto a pool of worker threads 
        // dedicated to polling futures that are ready to make progress. 
        handles.push(task::spawn(async move {
            cheapo_request(&host, port, &path).await
        }));
    }
    /*
    // async blocks
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
}
