use std::{fs::File, io::Write};

use rand::{Rng, distributions::Alphanumeric};


fn main() -> std::io::Result<()>{
    let mut rng = rand::thread_rng();

    for index in 1..101{
        
         let random_text: String = (0..100000)
            .map(|_| rng.sample(Alphanumeric) as char)
            .collect();

        let mut file = File::create(format!("documents/document_{}.txt", index))?;
        file.write_all(random_text.as_bytes())?;
        println!(" ({0}/100) Archivo generado >> documents/document_{0}.txt", index);
    }

    Ok(())
}