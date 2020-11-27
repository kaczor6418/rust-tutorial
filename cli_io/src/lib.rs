use std::error::Error;
use std::fs::read_to_string as read_from_file;

mod config;

pub use config::config::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = read_from_file(config.file_name)?;
    println!("File content:\n{}", content);
    Ok(())
}
