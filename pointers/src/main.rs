use crate::List::{Cons, Nil};
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: i32,
}

impl Person {
    pub fn new(first_name: String, last_name: String, age: i32) -> Self {
        Person {
            first_name,
            last_name,
            age,
        }
    }
}

impl Drop for Person {
    fn drop(&mut self) {
        println!("Person: {:?} has been drop!", self);
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        println!("MyBox Deref has been called!");
        return &self.0;
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m); // is equal hello(&(*m)[..]); because first we will take a value under *m address (String in our case) then we will take a slice of this data [..] so str and we will pass it as a reference &
    {
        let person = Person::new(String::from("Jan"), String::from("Kowalski"), 32);
        println!("Person instance has been created");
    }
    // Person drop should be called because person variable go out of the scope
    let person = Person::new(String::from("Justyna"), String::from("Kowalczyk"), 22);
    std::mem::drop(person); // person was dropped before going out of main (scope of person variable)
                            // Reference count
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    let c = Cons(4, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    println!("End of main!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
