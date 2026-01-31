fn main() {

    let mut i = 0;
    let incr = || {
        i += 1;
        println!("Ding! i is now: {}", i);
    };

    call_twice(incr);

    println!("✅ Finalizado!");

    fn call_twice<F>(mut closure: F) where F: FnMut(){
        closure();
        closure();
        closure();
    }
    
}
