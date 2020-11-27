use cli_io::{run, Config};
use std::env::args as cli_args;
use std::process::exit;

fn main() {
    let user_input: Vec<String> = cli_args().collect();
    let config = match Config::new(&user_input) {
        Ok(valid_config) => valid_config,
        Err(error_message) => {
            println!("{}", error_message);
            exit(1);
        }
    };
    println!("Searching for: {}", config.query);
    println!("In file: {}", config.file_name);
    match run(config) {
        Ok(()) => exit(0),
        Err(error) => {
            println!("Application has finished with error: {}", error);
            exit(2);
        }
    };
}
