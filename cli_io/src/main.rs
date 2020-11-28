use cli_io::run;
use std::env::args as cli_args;
use std::process::exit;

fn main() {
    let mut user_input: Vec<String> = cli_args().collect();
    user_input.remove(0);
    match run(&user_input) {
        Ok(()) => exit(0),
        Err(error) => {
            println!("Application has finished with error: {}", error);
            exit(2);
        }
    };
}
