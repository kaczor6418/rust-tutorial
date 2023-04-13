fn main() {
    let short_if = if is_bigger_than_10(1, false) { 12 } else { 10 };
    println!("short if expression: {}", short_if);
    is_bigger_than_10(1, true);
    is_bigger_than_10(10, true);
    is_bigger_than_10(12, true);
    while_loop();
    for_loop_array();
    for_loop_range();
    loop_return_value();
    labeled_loop();
    // infinite_loop();
}

fn is_bigger_than_10(value: i32, verbose: bool) -> bool {
    if verbose == false {
        return value > 10;
    }
    if value == 10 {
        println!("Value: {} is equal to 10", value);
    } else if value > 10 {
        println!("Value: {} is bigger to 10", value);
    } else {
        println!("Value: {} is smaller to 10", value);
    }
    println!("---------------------------------------");
    return value > 10;
}

fn while_loop() {
    println!("While loop");
    let mut i: i32 = 0;
    while i < 5 {
        println!("i: {}", i);
        i += 1;
    }
    println!("---------------------------------------");
}

fn for_loop_array() {
    println!("For loop of array");
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    for value in arr {
        println!("value: {}", value);
    }
    println!("---------------------------------------");
}

fn for_loop_range() {
    println!("For loop range");
    for value in (0..5).rev() {
        println!("value: {}", value);
        if value == 2 {
            break;
        }
    }
    println!("---------------------------------------");
}

fn loop_return_value() {
    println!("Loop and return");
    let mut counter = 0;
    let result = loop {
        if counter == 5 {
            break counter + 5;
        }
        counter += 1;
    };
    println!("Value returned from loop: {}", result);
    println!("---------------------------------------");
}

fn labeled_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
}

fn infinite_loop() {
    println!("Infinite loop");
    let mut i: i8 = 0;
    loop {
        println!("i: {}", i);
        i += 1;
    }
}
