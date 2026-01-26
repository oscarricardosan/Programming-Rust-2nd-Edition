use std::io::Write;
use std::fs::File;
use std::hash::Hash;
use std::fmt::Debug;

fn main() -> std::io::Result<()> {
    let mut local_file= File::create("hello.txt")?;
    say_hello(&mut local_file)?;
    println!("✅ Finalizado!");

    let mut bytes= vec![];
    say_hello(&mut bytes)?;
    assert_eq!(bytes, b"hello world\n");

    // El trait write_all funciona ya que se tiene el use
    // use std::io::Write;
    // Ya que el rasgo debe estar dentro del alcance
    let mut buf: Vec<u8> = vec![];
    buf.write_all(b"hello")?;


    let mut bytes= vec![];
    say_hello_generic_fun(&mut bytes)?;
    Ok(())
}

fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}

fn say_hello_generic_fun<W: Write>(out: &mut W) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}

// Un parametro que tenga el trait Debug para ser imprimible
// Eq para operaciones de comparación
// Hash opara creación de tablas
// u32 y String implementan los
// fn top_ten<T: Debug + Hash + Eq>(values: &Vec<T>)

//Función generica con múltiples parámetros
// fn run_query<M: Mapper + Serialize, R: Reducer + Serialize>(data: &DataSet, map: M, reduce: e)->Results

//Función generica con múltiples parámetros, usando el where
// para abreviar los trait que deben implementan los parametros
// fn run_query<M, R>(data: &DataSet, map: M, reduce: e)->Results
//      where M: Mapper + Serialize,
//            R: Reducer + Serialize {}

// Función generica con timelimes
// fn nearest<´t, 'c, P>(target: &'t P, candidates: &'c [P])-> &'c P
//    where P:MeasureDistance
//    {...}
