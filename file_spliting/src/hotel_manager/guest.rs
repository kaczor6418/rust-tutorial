pub struct Guest {
    first_name: String,
    last_name: String,
    age: i32,
}

impl Guest {
    pub fn new(first_name: String, last_name: String, age: i32) -> Guest {
        return Guest {
            first_name,
            last_name,
            age,
        };
    }

    pub fn show_guest_data(&self) {
        println!("First name: {}", self.first_name);
        println!("Last name: {}", self.last_name);
        println!("Age: {}", self.age);
    }
}
