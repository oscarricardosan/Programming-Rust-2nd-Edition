use std::{error::Error, sync::Arc, thread};
use rayon::prelude::*;

fn main() {
    let ignore_numbers= (10..=4900).collect::<Vec<i32>>();

    let worklists= (1..=5000).collect::<Vec<i32>>();

    let worklists: Vec<Vec<i32>> = worklists
        .chunks(8)
        .map(|chunk| chunk.to_vec())
        .collect();

    worklists.par_iter()
        .for_each(|worklist|{
            let valid_numbers=  worklist
                .iter()
                .filter(|number|{
                    !ignore_numbers.contains(number)
                }).collect::<Vec<&i32>>();

            if valid_numbers.len() > 0 {
                dbg!(valid_numbers);
            }
        });    
        
}
