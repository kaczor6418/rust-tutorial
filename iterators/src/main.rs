fn main() {
    let v1 = vec![1, 2, 3];
    print_vector_data(&v1);
}

fn print_vector_data(vector: &Vec<i32>) {
    let v1_iterator = vector.iter();
    for element in v1_iterator {
        println!("Vec element: {}", element);
    }
}
