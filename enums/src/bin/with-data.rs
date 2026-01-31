#[derive(Debug, PartialEq)]
#[allow(dead_code)]
enum TimeUnit {
    Days,
    Months,
    Years,
}

#[allow(dead_code)]
impl TimeUnit {
    fn plural(&self) -> &'static str {
        match self {
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }

    fn singular(&self) -> &'static str {
        self.plural().trim_end_matches('s')
    }
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
enum RoughtTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

//Enum con campos nombrados
#[derive(Debug)]
enum Point3d {
    Origin,
}
#[allow(dead_code)]
enum Shape {
    Sphere { center: Point3d, radius: f32 },
    Cuboid { corner1: Point3d, corner2: Point3d },
}

//Enum con los tres tipos
// Sin datos [Single...]
// tupla[ItsComplicated(Option<String>)]
// struct [ItsExtremelyComplicated { car: Point3d, cdr: Point3d }]
#[allow(dead_code)]
enum RelationshipStatus {
    Single,
    InARelationship,
    ItsComplicated(Option<String>),
    ItsExtremelyComplicated { car: Point3d, cdr: Point3d },
}

fn main() {
    let seven_yers = RoughtTime::InTheFuture(TimeUnit::Years, 7);
    dbg!(&seven_yers);

    let years: Option<TimeUnit> = match seven_yers {
        RoughtTime::InTheFuture(years, _) => Some(years),
        _ => None,
    };
    assert_eq!(years.unwrap().plural(), "years");

    let unit_sphere = Shape::Sphere {
        center: Point3d::Origin,
        radius: 1.0,
    };
    let center: Option<Point3d> = match unit_sphere {
        Shape::Sphere { center, radius: _ } => Some(center),
        _ => None,
    };

    dbg!(center);
    println!("");

    println!(
        " * {}",
        rought_time_to_english(&RoughtTime::InTheFuture(TimeUnit::Years, 1))
    );
    println!(
        " * {}",
        rought_time_to_english(&RoughtTime::InTheFuture(TimeUnit::Years, 3))
    );
    println!(" * {}", rought_time_to_english(&RoughtTime::JustNow));
    println!(
        " * {}",
        rought_time_to_english(&RoughtTime::InThePast(TimeUnit::Years, 3))
    );
    println!(
        " * {}",
        rought_time_to_english(&RoughtTime::InThePast(TimeUnit::Years, 1))
    );
    println!("");

    println!("✅ Finalizado!");
}

// Ejemplo de patron de coincidencia para extraer un valor de una enumeración
fn rought_time_to_english(rt: &RoughtTime) -> String {
    match rt {
        RoughtTime::InThePast(ref units, 1) => format!("a {} ago", units.singular()),
        RoughtTime::InThePast(ref units, ref count) => format!("{} {} ago", count, units.plural()),
        RoughtTime::JustNow => format!("juts now"),
        RoughtTime::InTheFuture(ref units, 1) => format!("a {} from now", units.singular()),
        RoughtTime::InTheFuture(ref units, ref count) => {
            format!("{} {} from now", count, units.plural())
        }
    }
}
