pub trait Shape {
    fn can_hold(&self, compared_shape: &dyn Shape) -> bool {
        return self.circuit() >= compared_shape.circuit();
    }
    fn circuit(&self) -> f64;
    fn area(&self) -> f64;
}
