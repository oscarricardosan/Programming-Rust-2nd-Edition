use std::thread;

fn main() {
    thread::spawn(|| {
        println!("hello from a child thread");
    }).join().unwrap();
}
