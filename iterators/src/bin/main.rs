use std::{ffi::OsStr, iter::from_fn, path::Path};
use rand::random;


fn main() {
    
    dbg!(triangle(3));

    let v= vec!["antimony", "arsenic", "alumim", "selenium"];
    
    // Iterador conversión inferida por rust
    println!("There's:");
    for element in &v {
        println!(" * {}", element);
    }

    // Iterador llamado directo
    let mut iterator= (&v).into_iter();
    while let Some(element) = iterator.next() {
        println!(" - {}", element);
    }

    // Iter and iter_mut
    let v= vec![4, 20, 12];
    let mut iterator= v.iter();
    assert_eq!(iterator.next(), Some(&4));
    assert_eq!(iterator.next(), Some(&20));
    assert_eq!(iterator.next(), Some(&12));
    assert_eq!(iterator.next(), None);

    //Iter de prestamo
    let path= Path::new("/mnt/tmp");
    let mut iterator= path.iter();
    assert_eq!(iterator.next(), Some(OsStr::new("/")));
    assert_eq!(iterator.next(), Some(OsStr::new("mnt")));
    assert_eq!(iterator.next(), Some(OsStr::new("tmp")));
    assert_eq!(iterator.next(), None);

    // (&collect).into_iter => Produce referencias compartidas a cada elemento al recorrerlo
    // for element in &collection
    // (&mut collect).into_iter => Produce referencias mutables a cada elemento al recorrerlo
    // for element in &mut collection
    // (collect).into_iter => Retorna la propiedad por valor a quien la
    // for element in collection


    let lengths: Vec<f64> = from_fn(||Some((random::<f64>() - random::<f64>()).abs()))
        .take(10)
        .collect();

    dbg!(lengths);


    dbg!(fibonacci().take(8).collect::<Vec<_>>());
    assert_eq!(
        fibonacci().take(8).collect::<Vec<_>>(), 
        vec![1, 1, 2, 3, 5, 8, 13, 21]
    );

    //Drenar elementos
    let mut outer= "Earth".to_string();
    let inner= String::from_iter(outer.drain(1..4));
    assert_eq!(outer, "Eh");
    assert_eq!(inner, "art");
    println!("✅ Finalizado!");

}


fn triangle(n: i32)-> i32 {
    (1..=n).fold(0, |sum, item| sum + item)
}

fn fibonacci() -> impl Iterator<Item = usize> {
    let mut state= (0, 1);
    std::iter::from_fn(move || {
        state= (state.1, state.0 + state.1);
        Some(state.0)
    })
   
}