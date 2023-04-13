fn main() {
    char_function('t');
    i32_function(12);
    time(12, "hours");
    println!("Expression implicitly: {}", expression_implicitly());
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

fn  time(value: i32, unit: &str) {
    println!("It is {} {}", value, unit);
}

// If last line of function doesn't contains semicolon it is treated as returned value
fn expression_implicitly() -> i32 {
    12
}
