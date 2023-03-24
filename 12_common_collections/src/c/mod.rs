// hashmaps
use std::collections::HashMap;

pub fn one(){
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores: HashMap<String,i32> = HashMap::new();
    scores.insert(blue, 10); // this will take ownership of our strings, this requires lifetimes
    scores.insert(yellow, 50);

    let team_name = String::from("Blue");
    match scores.get(&team_name) {
        None => { println!("None") }
        Some(value) => { println!("{}", value)}
    }
}

pub fn two(){
    let mut scores: HashMap<String,i32> = HashMap::new();

    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Blue"),20);
    // this does not override the value
    scores.entry(String::from("Yellow")).or_insert(30); // yellow = 30
    scores.entry(String::from("Yellow")).or_insert(40); // not doing anything since it exists
}

pub fn three() {
    let text = "hello world wonderful world";
    let mut map: HashMap<&str, i32> = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}