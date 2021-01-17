use std::ops::Deref;

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
    println!("End of main!");
}
