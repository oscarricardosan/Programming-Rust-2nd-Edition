struct City{
    name: String,
    monster_attack_risk: usize,
}

fn main() {

    let cities= vec![
        City{
            name: "Sogamoso".into(),
            monster_attack_risk: 10,
        },
        City{
            name: "Paipa".into(),
            monster_attack_risk: 1,
        },
        City{
            name: "Duitama".into(),
            monster_attack_risk: 0,
        }
    ];

    fn count_selected_cities(cities: &Vec<City>, test_fn: fn(&City)-> bool)-> usize{
        let mut count= 0;
        for city in cities{
            if test_fn(city){
                count+=1;
            }
        }
        count
    }

    fn has_monster_attacks(city: &City) -> bool {
        city.monster_attack_risk > 0
    }

    let n= count_selected_cities(&cities, has_monster_attacks);
    println!("Ciudades con ataques de monstruos {}", n);

    let n= count_selected_cities(&cities, |city|
        city.monster_attack_risk == 0
    );
    println!("Ciudades sin ataques de monstruos {}", n);

    println!("✅ Finalizado!");

}
