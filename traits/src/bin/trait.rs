use std::io::Write;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let mut local_file= File::create("hello.txt")?;
    say_hello(&mut local_file)?;
    println!("✅ Finalizado!");

    let mut bytes= vec![];
    say_hello(&mut bytes)?;
    assert_eq!(bytes, b"hello world\n");

    // El trait write_all funciona ya que se tiene el use
    // use std::io::Write;
    // Ya que el rasgo debe estar dentro del alcance
    let mut buf: Vec<u8> = vec![];
    buf.write_all(b"hello")?;

    Ok(())
}

fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}
