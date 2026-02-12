use std::collections::HashMap;

fn main() {
    type RoomId= String;
    type RoomExits= Vec<(char, RoomId)>;
    type RoomMap= HashMap<RoomId, RoomExits>;

    let mut map= RoomMap::new();
    map.insert("Cobble Crawl".to_string(), vec![
        ('W', "Debis Room".to_string())
    ]);
    map.insert("Debis Room".to_string(), vec![
        ('E', "Cobble Crawl".to_string()),
        ('W', "Sloping Canyon".to_string())
    ]);

    serde_json::to_writer(
        &mut std::io::stdout(), &map
    ).unwrap();


    println!();
    println!("✅ Finalizado!");
}
