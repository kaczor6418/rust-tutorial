use cli_io::run;
use std::env::args as cli_args;
use std::process::exit;

fn main() {
    let user_input: Vec<String> = cli_args().skip(1).collect();
    run(user_input).unwrap_or_else(|error| {
        eprintln!("Application has finished with error: {}", error.to_string());
        exit(2);
    })
}
