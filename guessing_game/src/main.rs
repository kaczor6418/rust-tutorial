use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let secret_number: i32 = rand::thread_rng().gen_range(1, 101);

    loop {
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let converted_guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match converted_guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };

        if converted_guess == secret_number {
            println!("You guess the number: {}", converted_guess);
        } else if secret_number < converted_guess {
            println!("Your number is too big: {}", converted_guess);
        } else {
            println!("Your number is too small: {}", converted_guess);
        }
    }
}
