use std::collections::HashMap;

fn main() {
    let mut vec: Vec<i32> = Vec::new();
    let mut infer_vec = vec![1, 2, 3];
    vec.push(4);
    vec.push(5);
    vec.push(6);
    println!("First elem of vec: {}", vec[0]);
    println!("First elem of vec: {}", infer_vec[0]);

    let vec_item_reference = &vec[2];

    match vec.get(2) {
        Some(vec_item_reference) => println!("Under 2nd index of vec you can find number 3"),
        None => println!("Under 2nd index of there is nothing"),
    }

    match vec.get(8) {
        Some(vec_item_reference) => println!("Under 2nd index of vec you can find number 3"),
        None => println!("Under 2nd index of there is nothing"),
    }

    for item in vec.iter() {
        println!("Vector item: {}", item);
    }

    // map values of vector
    for item in &mut vec {
        *item *= 2;
    }

    for item in vec.iter() {
        println!("Vector item: {}", item);
    }

    let mut s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    s1 = s1 + &s2;
    println!("s1: {}", s1);

    let first_name = String::from("Jan");
    let last_name = String::from("Kowalski");
    let full_name = format!("{} {}", first_name, last_name);
    println!("first name: {}", first_name);
    println!("last name: {}", last_name);
    println!("full name: {}", full_name);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 20);
    scores.entry(String::from("Blue")).or_insert(30);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in scores.iter() {
        println!("Key: {}, Value: {}", key, value);
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // .entry return mutable reference to value under founded key. If key doesn't exists in this map then entry will create new key with new value
        *count += 1; // here we have a reference to value founded under the key so we can mutate this value which will also effect value taken from hash map
    }

    println!("{:?}", map);

}
