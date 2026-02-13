use std::{io, net::TcpListener, thread::spawn};

fn echo_main(addr: &str) -> io::Result<()> {
    let listener= TcpListener::bind(addr)?;
    println!("listening on {}", addr);

    loop {
        let (mut stream, addr)= listener.accept()?;
        println!("Connection received from {}", addr);

        let mut write_stream= stream.try_clone()?;

        spawn(move || {
            io::copy(&mut stream, &mut write_stream)
                .expect("Error in client thread: ");

            println!("Connection closed\n\n")
        });

    }
}

fn main() {
    
    echo_main("0.0.0.0:17007").expect("Error: ");

}
