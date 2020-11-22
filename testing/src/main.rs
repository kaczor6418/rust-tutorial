pub mod shapes;
pub use shapes::rectangle::Rectangle;

/// this macro will let Rust know that there is a test module
#[cfg(test)]
mod rectangle_test {
    use crate::shapes::shape::Shape;
    use crate::Rectangle;

    /// this macro will let Rust know that this test should be executed if cargo test
    /// `assert!` macro test is code executed inside return true
    #[test]
    fn rectangle_with_smaller_circuit_should_not_fit_in_rectangle_with_bigger_circuit() {
        let smaller_rectangle = Rectangle::new(1.00, 2.00);
        let bigger_rectangle = Rectangle::new(2.00, 3.00);
        assert!(!smaller_rectangle.can_hold(&bigger_rectangle));
    }

    /// If we need more custom assert, then we can use `Result` enum to tell test when test result is ok and when it is not
    #[test]
    fn rectangle_with_smaller_circuit_should_fit_in_rectangle_with_bigger_circuit(
    ) -> Result<(), String> {
        let bigger_rectangle = Rectangle::new(2.00, 3.00);
        let smaller_rectangle = Rectangle::new(1.00, 2.00);
        return if bigger_rectangle.can_hold(&smaller_rectangle) {
            Ok(())
        } else {
            Err(String::from(
                "Bigger rectangle couldn't hold smaller rectangle",
            ))
        };
    }

    #[test]
    fn rectangle_with_circuit_should_fit_in_rectangle_with_same_circuit() {
        let rectangle_1 = Rectangle::new(2.00, 3.00);
        let rectangle_2 = Rectangle::new(2.00, 3.00);
        assert!(rectangle_1.can_hold(&rectangle_2));
    }

    #[test]
    fn circuit_should_be_counted_correctly() {
        let expected_circuit = 6.0;
        let rectangle = Rectangle::new(1.0, 2.0);
        assert_eq!(expected_circuit, rectangle.circuit());
    }

    #[test]
    fn circuit_of_two_different_rectangles_should_not_be_the_same() {
        let rectangle_1 = Rectangle::new(2.00, 3.00);
        let rectangle_2 = Rectangle::new(3.00, 4.00);
        assert_ne!(rectangle_1.circuit(), rectangle_2.circuit());
    }

    /// `should_panic` macro will let rust know that this test should throw an exception
    /// If we wont precise panic message then it can be any panic message
    #[test]
    #[should_panic = "I told you that i will panic!"]
    fn rectangle_should_panic_after_calling_will_panic() {
        let rectangle_1 = Rectangle::new(2.00, 3.00);
        rectangle_1.i_will_panic();
    }
}

fn main() {
    println!("Testing time!");
}
