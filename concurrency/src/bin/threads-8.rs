use std::{sync::Arc, thread};

fn main() {
    const NTHREADS:usize = 8;

    let ignore_numbers= (10..=4900).collect::<Vec<i32>>();
    let ignore_numbers= Arc::new(ignore_numbers);

    let worklists= (1..=5000).collect::<Vec<i32>>();
    let chunk_size = worklists.len().div_ceil(NTHREADS);
    
    let worklists = worklists
        .chunks(chunk_size)
        .map(|chunk| chunk.to_vec());

    let mut thread_handles= vec![];

    for worklist in worklists {

        //Esto solo clona el Arc y aumenta el conteo de referencias. 
        // No clona la data
        let ignore_numbers_for_child= ignore_numbers.clone();
        thread_handles.push(thread::spawn(move||{
            
            let valid_numbers=  worklist.iter()
                .filter(|number|{
                    !ignore_numbers_for_child.contains(number)
                }).collect::<Vec<&i32>>();

            if valid_numbers.len() > 0 {
                dbg!(valid_numbers);
            }
        }));
    }

    for handle in thread_handles {
        handle.join().unwrap();
    }
    
}
