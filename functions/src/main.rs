fn main() {
    char_function('t');
    i32_function(12);
    let x = {
        let y = 3;
        y + 1
    };
    println!("x: {}", x);
}

// this method will print and return char which you passed as parameter
fn char_function(value: char) -> char {
    println!("char value: {}", value);
    return value;
}

// this method will print and return i32 which you passed as parameter
fn i32_function(value: i32) -> i32 {
    println!("i32 value: {}", value);
    return value;
}
