pub struct Config {
    pub query: String,
    pub file_name: String,
    pub insensitive: bool,
}

impl Config {
    pub const INSENSITIVE: &'static str = "--insensitive";
    pub fn new(user_input: &Vec<String>) -> Result<Config, &'static str> {
        let mut user_input_iterator = user_input.iter();
        let query = match user_input_iterator.next() {
            Some(arg1) => String::from(arg1),
            None => return Err("You have to provide query string"),
        };
        let file_name = match user_input_iterator.next() {
            Some(arg2) => String::from(arg2),
            None => return Err("You have to provide file name"),
        };
        let insensitive = match user_input_iterator.next() {
            Some(arg3) => {
                if arg3 == Config::INSENSITIVE {
                    true
                } else {
                    return Err("Invalid flag");
                }
            }
            None => false,
        };
        if user_input_iterator.next().is_some() {
            return Err("Too many arguments");
        }
        return Ok(Config {
            query,
            file_name,
            insensitive,
        });
    }
}

#[cfg(test)]
mod config_test;
