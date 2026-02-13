use std::{fs::OpenOptions, io::Write};

macro_rules! log {
    ($format:tt, $($arg:expr), *) => (
        write_log_entry(format_args!($format, $($arg), *))
    )
}

fn main() {
    
    log!("Hark! {:#?}\n", 2);

    println!("✅ Finalizado!");
}

fn write_log_entry(entry: std::fmt::Arguments){
    if loggin_enabled() {
        let mut log_file= OpenOptions::new()
            .append(true)
            .create(true)
            .open("log-file")
            .expect("Failed to open file");

        log_file.write_fmt(entry)
            .expect("Failed to write to log")
    }
}

fn loggin_enabled() -> bool{
    true
}
