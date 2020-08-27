fn main() {
    let mut some_string: String = String::from("Is mutable string?");
    let mut first_mutable_reference_of_some_string: &mut String = &mut some_string;
//    let mut second_mutable_reference_of_some_string2: String = some_string; Can not create to mutable references of the same variable
    println!("Before mutation: {}", some_string);
    change(&mut some_string);
    println!("After mutation: {}", some_string);

    let mut s = String::from("hello");
    let r1 = &s;
//    let r2 = &mut s; Can not create mutable reference if there is already another pointer which is referring to this variable pointer
//    println!("{}, and {}", r1, r2);
}

fn change(some_string: &mut String) {
    some_string.push_str(" -> yes I am mutable");
}
