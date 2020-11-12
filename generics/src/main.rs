enum Option<T> {
    SOME(T), // if there will be a value, value will be typed as T
    NOE,
}

enum Result<T, E> {
    OK(T),    // if operation will not throw an error you will get value typed as T
    ERROR(E), // if operation will occurred an error you will get value typed as E (special error type)
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn new(x: T, y: U) -> Point<T, U> {
        return Point { x, y };
    }
}

impl Point<f64, f64> {
    fn get_distance(&self, point_b: &Point<f64, f64>) -> f64 {
        return ((point_b.x - self.x).powf(2.0) + (point_b.y - self.y).powf(2.0)).sqrt();
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// fn get_distance<T>(point_a: &Point<T>, point_b: &Point<T>) -> T {}

fn main() {
    let integer_point = Point::new(5, 12);
    let float_integer_point = Point::new(5, 12.1);
    let float_point_1 = Point::new(5.12, 12.32);
    let float_point_2 = Point::new(4.12, 8.32);
    println!(
        "Distance between two points: {}",
        float_point_1.get_distance(&float_point_2)
    );
    // Traits bonds + generics
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
