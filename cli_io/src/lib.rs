use std::error::Error;
use std::fs::read_to_string as read_from_file;

mod config;

mod search_engine;
use config::config::Config;
use search_engine::search_engine::SearchEngine;

pub fn run(user_input: Vec<String>) -> Result<(), Box<dyn Error>> {
    let config = Config::new(&user_input)?;
    let content = read_from_file(config.file_name)?;
    let search_engine = SearchEngine::new(&config.query, &content);
    let search_result: Vec<&str> = if config.insensitive && config.regex {
        search_engine.regex_search(true)
    } else if config.regex {
        search_engine.regex_search(false)
    } else if config.insensitive {
        search_engine.insensitive_search()
    } else {
        search_engine.search()
    };
    for single_result in search_result {
        println!("{}", single_result);
    }
    Ok(())
}
