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

}
