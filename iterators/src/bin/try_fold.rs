use std::error::Error;
use std::str::FromStr;
use std::io::prelude::*;

fn main()-> Result<(), Box<dyn Error>>{
    
    let stdin= std::io::stdin();
    let sum= stdin.lock()
        .lines()
        .try_fold(0, |sum, line|-> Result<u64, Box<dyn Error>>{
            Ok(sum + u64::from_str(&line?.trim())?)
        })?;

    println!("{}", sum);
    println!("✅ Finalizado!");

    Ok(())
}