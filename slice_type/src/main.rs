fn main() {
    let sentence: String = String::from("Hello my name is Jack");
    let first_word: &str = find_first_word(&sentence);
//    sentence.clear(); Will throw an error because we already created a variable with reference to sentence so we can not mutate this variable after setting a reference to other variable
    println!("First word: {}", first_word);
    let some_numbers: [i8; 5] = [1, 2, 3, 4, 5];
    println!("Only firs 3 numbers of array: [1, 2, 3, 4, 5]");
    for (i, &value) in some_numbers[..3].iter().enumerate() {
        println!("Value under index: {} is number: {}", i, value);
    }
}

fn find_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
