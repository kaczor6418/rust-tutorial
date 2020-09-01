#[derive(Debug)]
struct User {
    first_name: String,
    last_name: String,
    age: i32,
}

impl User {
    fn new(first_name: &str, last_name: &str, age: i32) -> User {
        return User {
            first_name: String::from(first_name),
            last_name: String::from(last_name),
            age,
        };
    }

    fn full_name(&self) -> String {
        return format!("{} {}", self.first_name, self.last_name);
    }

    fn is_older(&self, user: &User) -> bool {
        return self.age > user.age;
    }

    fn change_age(&mut self, new_age: i32) {
        self.age = new_age;
    }
}

fn main() {
    let mut user1: User = User::new("John", "Doe", 22);
    let mut user2: User = User::new("Jack", "Pink", 25);
    let user1_full_name: String = user1.full_name();
    println!("User full name: {}", user1_full_name);
    println!("User1 struct: {:#?}", user1);
    println!("User2 struct: {:#?}", user2);
    println!("Is User1 older than User2: {}", user1.is_older(&user2));
    println!("Is User2 older than User1: {}", user2.is_older(&user1));
    user1.change_age(30);
    println!("User1 struct: {:#?}", user1);
    println!("Is User1 older than User2: {}", user1.is_older(&user2));
    println!("Is User2 older than User1: {}", user2.is_older(&user1));
}
