fn main() {
    println!("{}", min(10, 3));
}

// <T: Ord> en esta función significa que min puede ser usardo
// con argumentos de cualquier tipo T que implementen
// el trait std::cmp::Ord, que es, cualquier tipo ordenado (que pueda hacer comparación de < > =).
fn min<T: std::cmp::Ord>(value1: T, value2: T) -> T{
    if value1 <= value2{
        value1
    }else{
        value2
    }
}
