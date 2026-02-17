use std::{thread};

fn main() {
    const NTHREADS:usize = 8;
    let worklists: Vec<Vec<i32>> = (1..=50000).collect::<Vec<i32>>()
        .chunks(NTHREADS)
        .map(|chunk| chunk.to_vec())
        .collect();

    let mut thread_handles= vec![];

    for worklist in worklists {
        thread_handles.push(thread::spawn(move||{
            dbg!(worklist)
        }));
    }

    for handle in thread_handles {
        handle.join().unwrap();
    }
    

}
