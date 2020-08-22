fn main() {
    is_bigger_than_10(1);
    is_bigger_than_10(10);
    is_bigger_than_10(12);
    while_loop();
    for_loop_array();
    for_loop_range();
    infinite_loop();
}

fn is_bigger_than_10(value: i32) {
    if value == 10 {
        println!("Value: {} is equal to 10", value);
    } else if value > 10 {
        println!("Value: {} is bigger to 10", value);
    } else {
        println!("Value: {} is smaller to 10", value);
    }
    println!("---------------------------------------");
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
    for value in arr.iter() {
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

fn infinite_loop() {
    println!("Infinite loop");
    let mut i: i8 = 0;
    loop {
        println!("i: {}", i);
        i += 1;
    }
}
