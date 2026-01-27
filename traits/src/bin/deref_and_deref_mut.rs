use std::{fmt::Display, ops::{Deref, DerefMut}, vec};

struct Selector<T> {
    elements: Vec<T>,
    current: usize
}

impl <T> Deref for Selector<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.elements[self.current]
    }

}

impl <T> DerefMut for Selector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elements[self.current]
    }
}

fn main(){

    let mut s= Selector{
        elements: vec!['x', 'y', 'z'],
        current: 2
    };

    assert_eq!(*s, 'z');
    assert!(s.is_alphabetic());

    *s= 'w';
    assert_eq!(s.elements, ['x', 'y', 'w']);

    let s= Selector{
        elements: vec!["good", "bad", "ugly"],
        current: 2
    };

    show_it(&s);
    //show_it_generic(&s);//falla, struct Selector no implementa Display aunque sus elementos sean &str

    show_it_generic(&s as &str);//Fuerza la desreferenciación antes de pasar como parametro, &str si usa Display

    show_it_generic(&*s);//Fuerza la desreferenciación antes de pasar como parametro, &str si usa Display

    println!("✅ Finalizado!");
}

fn show_it(thing: &str) {
    println!(" -- {}", thing);
}

fn show_it_generic<T: Display + ?Sized>(thing: &T) {
    println!(" == {}", thing);
}
