use async_std::task;

fn main()
{
    let input= async_std::io::stdin();
    let future= async {
        let mut line= String::new();

        input.read_line(&mut line).await?;

        println!("Read line: {}", line);

        Ok::<(), std::io::Error>(())
    };

    println!("Write text");

    let response= task::block_on(future).unwrap();
    
    dbg!(response);

}

