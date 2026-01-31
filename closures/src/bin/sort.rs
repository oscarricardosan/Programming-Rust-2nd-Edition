#[derive(Debug, Clone)]
struct City{
    name: String,
    population: i64,
}


fn main() {
    let mut cities_list= vec![
        City{
            name: "Sogamoso".into(),
            population: 160_000,
        },
        City{
            name: "Paipa".into(),
            population: 1_000,
        },
        City{
            name: "Duitama".into(),
            population: 165_000,
        }
    ];

    let mut cities_list2= cities_list.clone();

    sort_cities_desc(&mut cities_list);
    dbg!(cities_list);

    sort_cities_asc(&mut cities_list2);
    //cities_list2.reverse();
    dbg!(cities_list2);
}


fn city_population_descending(city: &City) -> i64{
    city.population //negativo para el orden desc
}

fn sort_cities_desc(cities: &mut Vec<City>) {
    cities.sort_by_key(city_population_descending);
}

fn sort_cities_asc(cities: &mut Vec<City>) {
    cities.sort_by_key(|city| -city.population);
}
