fn main() {
    variables();
    data_types();
}

fn variables() {
    const X: u8 = 5;
    println!("x: {}", X);
    let mut y = 5;
    println!("y: {}", y);
    y = 4;
    println!("y: {}", y);

    let z = 1;
    println!("z: {}", z);
    let z = 0.1 + 0.2;
    println!("z: {}", z);
}


fn data_types() {
    let tup: (i32, char, f32) = (200, 'a', 21.32);
    println!("tup i32: {}", tup.0);
    println!("tup char: {}", tup.1);
    println!("tup i32: {}", tup.2);

    let arr: [i32; 4] = [1, 2, 3, 4];
    println!("arr length: {}", arr.len());
    println!("arr idx 0: {}", arr[0]);
    println!("arr idx 1: {}", arr[1]);
    println!("arr idx 2: {}", arr[2]);
    println!("arr idx 3: {}", arr[3]);

    // Fill array with given number [5, 5, 5]
    let same_val_arr: [i32; 3] = [5; 3];
    println!("tuple idx 0: {}", same_val_arr[0]);
    println!("tuple idx 1: {}", same_val_arr[1]);
    println!("tuple idx 2: {}", same_val_arr[2]);
}
