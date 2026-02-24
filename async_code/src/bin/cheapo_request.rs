use std::vec;
use async_std::io::prelude::*;
use async_std::net;


fn main()-> std::io::Result<()> {
    use async_std::task;
    println!("#### 1");
    let response= task::block_on(cheapo_request(
        "savne.net", 443, "/"
    ))?;
    
    println!("{}", response);



    let pages= vec![
        ("savne.net".to_string(), 443, "/1".to_string()),
        ("savne.net".to_string(), 443, "/2".to_string()),
        ("savne.net".to_string(), 443, "/3".to_string()),
        ("savne.net".to_string(), 443, "/4".to_string()),
        ("savne.net".to_string(), 443, "/5".to_string()),
        ("savne.net".to_string(), 443, "/6".to_string()),
    ];
    println!("#### 2");
    let results= task::block_on(many_requests(pages));

    for result in  results{

        println!("---");
        match result {
            Ok(response)=> println!("{}", response),
            Err(err)=> println!(" ** ERROR: {}", err),
        }
        println!("---");
    }
    println!("#### 3");
    
    Ok(())
}

async fn cheapo_request(
    host: &str, port: u16, path: &str 
)-> std::io::Result<String>
{
    let mut socket= net::TcpStream::connect((host, port)).await?;
    
    let request= format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);

    socket.write_all(request.as_bytes()).await?;
    socket.shutdown(net::Shutdown::Write)?;

    let mut response= String::new();
    socket.read_to_string(&mut response).await?;

    let response= format!("{}\n{}", &path, response);
    Ok(response)
}

async fn many_requests(
    requests: Vec<(String, u16, String)> 
)-> Vec<std::io::Result<String>>
{
    use async_std::task;
    
    let mut handles=  vec![];

    for (host, port, path) in requests {
        handles.push(task::spawn_local(async move {
            cheapo_request(&host, port, &path).await
        }));
    }

    let mut results= vec![];

    for handle in handles {
        results.push(handle.await);
    }

    results
}

