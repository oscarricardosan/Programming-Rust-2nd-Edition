use num::Complex;
use std::str::FromStr;

/// Parse the string `s` as a coordinate pais, like `"400x600"` or `"0.0,0.5"`.
/// 
/// Specifically, `s` should have the form <left><sep><right>, where <sep> is 
/// the character given by the `separator` argument, and <left> and <right> are
/// both string that can be parsed by `T::from_str`. `separator` must be aun
/// ASCII character.
/// 
/// If `s` has the proper form, return `Some<(x, y)>`. If it doesn't parse
/// correctly, return `None`.
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r))=> Some((l ,r)),
                _ => None
            }
        }
    }
}

fn main(){
    let text = "Hello, Rust!";
    let index_of_r = text.find('R');
    let index_of_r= index_of_r.unwrap();
    
    for i in 0..=index_of_r {
        print!("{} ", i);
        println!("{}", text.chars().nth(i).unwrap());
    }
    println!("--{}", text.len());
    
    for i in (index_of_r+1)..text.len(){
        print!("{} ", i);
        println!("{}", text.chars().nth(i).unwrap());
    }
    
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>(" ", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    
}



/// Try to determine if `c` is in the Mandelbrot set, using at most `limit`
/// iterations to decide.
///
/// If `c` is not a member, return `Some(i)`, where `i` is the number of
/// iterations it took for `c` to leave the circle of radius 2 centered on the 
/// origin. If `c` seems to be a member (more precisely, if we reached the 
/// iteration limit without being able to prove that `c` is not a member),
/// retutn `None`.
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0};
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}
