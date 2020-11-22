use crate::shapes::shape::Shape;

/// macro `derive(PartialEq, Debug)` powduje to że jeżeli w teście wystąpi błąd to zostaną wydrukowane wartości przypisane do strktury (o ile sa typu *primitive*)
/// jeżeli atrybuty strktury ą bardziej skomplikowanymi typami wtedy musimy zaimplementowąc `PartialEq` i `Debug` traits w naszej strukturze
#[derive(PartialEq, Debug)]
pub struct Rectangle {
    a: f64,
    b: f64,
}

impl Rectangle {
    pub fn new(a: f64, b: f64) -> Rectangle {
        return Rectangle { a, b };
    }

    pub fn i_will_panic(&self) {
        panic!("I told you that i will panic!");
    }
}

impl Shape for Rectangle {
    fn circuit(&self) -> f64 {
        return self.a * 2.0 + self.b * 2.0;
    }

    fn area(&self) -> f64 {
        return self.a * self.b;
    }
}
