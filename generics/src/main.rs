struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Point<T> {
        return Point {
            x,
            y,
        };
    }

    pub fn x_difference(&self, target_point: Point<T>) -> T {
        return (self.x - target_point.x).abs();
    }

    pub fn y_difference(&self, target_point: Point<T>) -> T {
        return (self.y - target_point.y).abs();
    }
}

fn main() {
    //traits
//    let mut numbers: Vec<i32> = vec![1, 12, 83, 32, 5];
//    let mut characters: Vec<char> = vec!['b', 'c', 'd', 'a'];
//    println!("Largest number: {}", largest(&numbers));
//    println!("Largest character: {}", largest(&characters));
    let int_point1: Point<i32> = Point::new(12, 11); // Coordinates of this point can be only integer values
    let int_point2: Point<i32> = Point::new(-52, 41);
    let floating_point1: Point<f32> = Point::new(12.32, 54.11); // Coordinates of this point can be only floats values
    let floating_point2: Point<f32> = Point::new(14.32, -14.11);
    println!("X difference between int points: {:?}", int_point1.x_difference(int_point2));
    println!("Y difference between int points: {:?}", int_point1.y_difference(int_point2));
    println!("X difference between floating points: {:?}", floating_point1.x_difference(floating_point2));
    println!("Y difference between floating points: {:?}", floating_point1.y_difference(floating_point2));
}
