struct Square {
    a: f64,
}

struct Rectangle {
    a: f64,
    b: f64,
}

struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

struct Cube {
    a: f64,
    b: f64,
    c: f64,
}

struct Person {
    name: String,
    age: i32,
}

/* Traits are kind of interfaces in which we can specify behaviour of structure. But there is some differences
In traits we can declare only a signature of method (without implementation)
Differences:
  - can not declare attributes
  - in shape implementation you can not declare other method. This implementation can contains only **`trait members`**
  - `+` is kind of intersection of two or more Traits it means that object has to be both or more traits in the same time
  - `where` to make code more clean if **Intersection** typeis long and use in more generics we can use where after method declaration
  - if return type of method is a `impl Trait` then type which implements this trait want be allowed as return type
*/

trait Shape {
    fn common_method(&self) -> String {
        return String::from("I am a Shape");
    }
    fn circuit(&self) -> f64;
    fn area(&self) -> f64;
}

trait Shape3D {
    fn volume(&self) -> f64;
}

impl Shape for Square {
    fn circuit(&self) -> f64 {
        return self.a * 4.0;
    }

    fn area(&self) -> f64 {
        return self.a.powi(2);
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

impl Shape for Triangle {
    fn circuit(&self) -> f64 {
        return self.a + self.b + self.c;
    }

    fn area(&self) -> f64 {
        let p = (self.a + self.b + self.c) / 2.0;
        return (p * (p - self.a) * (p - self.b) * (p - self.c)).sqrt();
    }
}

impl Shape for Cube {
    fn common_method(&self) -> String {
        return String::from("I am a 3D Shape");
    }

    fn circuit(&self) -> f64 {
        return self.a * 4.0 + self.b * 4.0 + self.c * 4.0;
    }

    fn area(&self) -> f64 {
        return self.a * self.b * 2.0 + self.a * self.c * 2.0 + self.b * self.c * 2.0;
    }
}

impl Shape3D for Cube {
    fn volume(&self) -> f64 {
        return self.a * self.b * self.c;
    }
}

fn accepts_only_shapes(shape: &impl Shape) {
    println!("{}", shape.common_method())
}

fn accepts_only_3d_shapes<T: Shape + Shape3D>(shape_3d: &T) {
    println!("{}", shape_3d.common_method())
}

fn accepts_only_3d_shapes_with_where<T>(shape_3d: &T)
where
    T: Shape + Shape3D,
{
    println!("{}", shape_3d.common_method())
}

// trait bound syntax

fn main() {
    let square = Square { a: 2.0 }; // circuit: 8, area: 4
    let rectangle = Rectangle { a: 2.0, b: 3.0 }; // circuit: 10, area: 6
    let triangle = Triangle {
        a: 3.0,
        b: 4.0,
        c: 5.0,
    }; // circuit: 12, area: 6
    let cube = Cube {
        a: 2.0,
        b: 3.0,
        c: 4.0,
    }; // circuit: 36, area: 52, volume: 24
    let person = Person {
        name: String::from("Jan"),
        age: 32,
    };
    println!(
        "Square → Circuit: {}, Area: {}",
        square.circuit(),
        square.area()
    );
    println!(
        "Rectangle → Circuit: {}, Area: {}",
        rectangle.circuit(),
        rectangle.area()
    );
    println!(
        "Triangle → Circuit: {}, Area: {}",
        triangle.circuit(),
        triangle.area()
    );
    println!(
        "Cube → Circuit: {}, Area: {}, Volume: {}",
        cube.circuit(),
        cube.area(),
        cube.volume()
    );
    accepts_only_shapes(&square);
    accepts_only_shapes(&rectangle);
    accepts_only_shapes(&triangle);
    // accepts_only_shapes(&person); will trow this error: the trait `Shape` is not implemented for `Person`
    accepts_only_3d_shapes(&cube);
    accepts_only_3d_shapes_with_where(&cube);
    // accepts_only_3d_shapes(&triangle); will throw this error: the trait `Shape3D` is not implemented for `Triangle`
}
