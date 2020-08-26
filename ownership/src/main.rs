fn main() {
    let some_string: String = String::from("Takes ownership");
    takes_ownership(some_string); // can not use variables after moving ownership
//    println!("Some string: {}", some_string1); → this will not work because ownership of some_string was moved to takes_ownership method
    let some_string2: String = gives_ownership();
    println!("Some string 2: {}", some_string2);
    let some_string3: String = takes_and_gives_ownership(String::from("Takes and gives back ownership"));
    println!("Some string 3: {}", some_string3);

    let s1: &str = "test";
    let s2: &str = s1;
    println!("s1: {}, s2: {}", s1, s2);

}

fn takes_ownership(some_string: String) {
    println!("Some string in function: {}", some_string);
}

fn gives_ownership() -> String {
    return String::from("Gives ownership");
}

fn takes_and_gives_ownership(some_string: String) -> String {
    return  some_string;
}
