mod bomb;
use bomb::Bomb;

fn main() {
    println!("----------------------------------------- v1");
    let v1 = vec![1, 2, 3];
    print_vector_data_with_for(&v1);
    print_vector_data_with_next(&v1);
    println!("----------------------------------------- v2");
    let increment_by_1 = |x: &i32| x + 1;
    let v2 = v1.iter().map(increment_by_1).collect();
    print_vector_data_with_for(&v2);
    println!("----------------------------------------- v3");
    let only_even_number = |x: &i32| x % 2 == 0;
    let v3 = v1
        .iter()
        .map(increment_by_1)
        .filter(only_even_number)
        .collect();
    print_vector_data_with_for(&v3);
    println!("----------------------------------------- Bomb");
    let bomb = Bomb::new(10);
    for time in bomb {
        println!("Tick, Took: {}", time);
    }
}

fn print_vector_data_with_for(vector: &Vec<i32>) {
    let v1_iterator = vector.iter();
    for element in v1_iterator {
        println!("Vec element: {}", element);
    }
}

fn print_vector_data_with_next(vector: &Vec<i32>) {
    let mut vector_iterator = vector.iter();
    println!("Vec element: {}", vector_iterator.next().unwrap());
    println!("Vec element: {}", vector_iterator.next().unwrap());
    println!("Vec element: {}", vector_iterator.next().unwrap());
    println!(
        "End of vector should finish with 0: {}",
        vector_iterator.next().unwrap_or_else(|| &0)
    );
}
