fn main() {
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

fn _old3() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry("Blue".to_string()).or_insert(50);
    scores.entry("Yellow".to_string()).or_insert(50);
    println!("{:?}", scores);
}

fn _old2() {
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map: HashMap<String, String> = HashMap::new();
    map.insert(field_name.clone(), field_value.clone());
    println!("{} : {}", field_name, field_value);

    println!("{:?}", map.get("Favorite ccolor").unwrap());
}

fn _old() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{} scored {} points", team_name, score);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
