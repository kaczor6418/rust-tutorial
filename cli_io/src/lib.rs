use std::error::Error;
use std::fs::read_to_string as read_from_file;
use std::process::exit;

mod config;
mod search_engine;

use config::config::Config;
use search_engine::search_engine::SearchEngine;

pub fn run(user_input: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let config = match Config::new(&user_input) {
        Ok(valid_config) => valid_config,
        Err(error_message) => {
            println!("{}", error_message);
            exit(2);
        }
    };
    println!("Searching for: {}", config.query);
    println!("In file: {}", config.file_name);
    let content = read_from_file(config.file_name)?;
    let search_engine = SearchEngine::new(&config.query, &content);
    let search_result: Vec<&str> = if config.insensitive {
        search_engine.search()
    } else {
        search_engine.insensitive_search()
    };
    for single_result in search_result {
        println!("{}", single_result);
    }
    Ok(())
}
