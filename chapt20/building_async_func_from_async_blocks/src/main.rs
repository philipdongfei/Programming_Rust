
use async_std::io::prelude::*;
use async_std::net;
use std::io;
use std::future::Future;

//TODO: fix
fn cheapo_request(host: & str, port: u16, path: &str)
   -> impl Future<Output = io::Result<String>> + 'static
{
    let host = host.to_string();
    let path = path.to_string();

    async move {
        let mut socket = net::TcpStream::connect((&*host, port)).await?;

        let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);
        socket.write_all(request.as_bytes()).await?;
        socket.shutdown(net::Shutdown::Write)?;

        let mut response = String::new();
        socket.read_to_string(&mut response).await?;

        Ok(response)

    }
}

async fn many_requests(requests: Vec<(String, u16, String)>)
                -> Vec<std::io::Result<String>>
{
    use async_std::task;

    let mut join_handles = vec![];

    for (host, port, path) in requests {

        join_handles.push(task::spawn_local(
                cheapo_request(&host, port, &path).await
        ));
    }

    let mut results = vec![];
    for join_handle in join_handles {
        let response = join_handle.await;
        let response = std::future::ready(response);
        results.push(response);
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
