use std::cmp::Ordering;

// Binario con enumeraciones al estilo C
//
// Enum sin valores
#[derive(Debug, PartialEq)]
enum Pet {
    Orca,
    Giraffe,
}

//Enum con valores
#[derive(Debug, PartialEq)]
enum HttpStatus {
    Ok = 200,
    NotModified = 304,
    NotFound = 404,
}

// Las enum pueden tener implementaciones:
#[derive(Debug, PartialEq)]
enum TimeUnit {
    Days,
    Months,
    Years,
}

impl TimeUnit {
    // Return the plurar noun for this time unit.
    fn plural(self) -> &'static str {
        match self {
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }

    fn singular(self) -> &'static str {
        self.plural().trim_end_matches('s')
    }
}

fn main() {
    assert_eq!(compare(1, 5), Ordering::Less);
    assert_eq!(compare(10, 5), Ordering::Greater);
    assert_eq!(compare(5, 5), Ordering::Equal);

    assert_ne!(Pet::Orca, Pet::Giraffe);

    // Las enumeraciones pueden convertirse a otros valores, más no al réves
    assert_eq!(HttpStatus::Ok as i32, 200);

    // Para conversión de valores a enum debe crearse una función personalizada
    assert_eq!(http_status_from_u32(404).unwrap(), HttpStatus::NotFound);

    assert_eq!(TimeUnit::Days.plural(), "days");
    assert_eq!(TimeUnit::Months.plural(), "months");
    assert_eq!(TimeUnit::Years.singular(), "year");

    println!("✅ Finalizado!");
}

fn compare(n: i32, m: i32) -> Ordering {
    if n < m {
        return Ordering::Less;
    }
    if n > m {
        return Ordering::Greater;
    }
    Ordering::Equal
}

fn http_status_from_u32(n: u32) -> Option<HttpStatus> {
    match n {
        200 => Some(HttpStatus::Ok),
        304 => Some(HttpStatus::NotModified),
        404 => Some(HttpStatus::NotFound),
        _ => None,
    }
}
