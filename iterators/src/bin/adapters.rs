
use std::{collections::{BTreeMap, HashMap}, str::FromStr, vec};

fn main() {
    
    let text= "    ponies   \n      giraffes\niguanas\nsquid".to_string();

    let v:Vec<&str>= text.lines()
        .map(str::trim)
        .collect();

    assert_eq!(v, ["ponies", "giraffes", "iguanas", "squid"]);

    let v:Vec<&str>= text.lines()
        .map(str::trim)
        .filter(|s|*s != "iguanas")
        .collect();

    assert_eq!(v, ["ponies", "giraffes","squid"]);

    let text= "1\nfrond .25 289\n3.1415 estuaty\n";
    for number in text
        .split_whitespace()
        .filter_map(|w|f64::from_str(w).ok())
    {
        println!("{:4.2}", number.sqrt())   
    }

    let mut major_cities= HashMap::new();
    major_cities.insert("Japan", vec!["Tokio", "Kyoto"]);
    major_cities.insert("United States", vec!["Portland", "Nashville"]);
    major_cities.insert("Brazil", vec!["Sao Paulo", "Brasilia"]);
    major_cities.insert("Kenya", vec!["Nairobi", "Mombasa"]);
    major_cities.insert("The Netherlands", vec!["Amsterdan", "Utrecht"]);

    let countries= ["Japan", "Brazil", "Kenya"];

    dbg!(
        countries.iter()
        .flat_map(|country|  &major_cities[country])
        .collect::<Vec<&&str>>()
    );
    for &city in countries.iter()
        .flat_map(|country|  &major_cities[country])
    {
        println!(" -> {}" ,city);
    }

    let mut parks= BTreeMap::new();
    parks.insert("Portland", vec!["Mt. Tabor Park", "Forest Park"]);
    parks.insert("Kyoto", vec!["Tadasu-no-Mori Forest", "Maruyama Koen"]);
    parks.insert("Nashville", vec!["Percy Warner Park", "Dragon Park"]);

    let all_parks:Vec<_>= parks.values().flatten().cloned().collect();
    assert_eq!(all_parks, vec![
        "Tadasu-no-Mori Forest", "Maruyama Koen",
        "Percy Warner Park", "Dragon Park",
        "Mt. Tabor Park", "Forest Park"
    ]);

    assert_eq!(
        vec![None, Some("day"), None, Some("one")]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>(),
        vec!["day", "one"]
    );

    assert_eq!(
        "chars".to_string().chars().flat_map(char::to_uppercase).map(|c| c.to_string()).collect::<Vec<_>>().join(""),
        "CHARS"
    );

    let message= "To jimb\r
From: superego <editor@oreilly.com>\r
\r
Did you get any writing done today?\r
When will you stop wasting time plotting fractals?\r";

    for header in message.lines().take_while(|l|!l.is_empty()).skip(1){
        println!("{}", header);
    }

    for body in message.lines().skip_while(|l|!l.is_empty()){
        println!("{}", body);
    }

}
