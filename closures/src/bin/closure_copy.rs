fn main() {

    let y = 10;
    let add_y= |x| x + y;
    let copy_of_add_y = add_y;

    assert_eq!(add_y(copy_of_add_y(22)), 42);

    println!("✅ Finalizado 1!");

    let mut greeting= String::from("Hello, ");
    let greet= move |name| {
        greeting.push_str(name);
        println!("{}", greeting);
    };

    greet.clone()("Alfred");
    greet.clone()("Bruce");

    println!("✅ Finalizado!");

}
