pub struct Config {
    pub query: String,
    pub file_name: String,
    pub insensitive: bool,
    pub regex: bool,
}

impl Config {
    pub const INSENSITIVE: &'static str = "--insensitive";
    pub const REGEX: &'static str = "--regex";
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
        let mut insensitive = false;
        let mut regex = false;
        let mut assign_flag_or_throw_error = |arg: &String| {
            return if arg == Config::INSENSITIVE {
                insensitive = true;
                Ok(())
            } else if arg == Config::REGEX {
                regex = true;
                Ok(())
            } else {
                Err("Invalid flag")
            };
        };
        match user_input_iterator.next() {
            Some(arg3) => {
                if assign_flag_or_throw_error(arg3).is_err() {
                    return Err("Invalid flag");
                }
            }
            None => (),
        };
        match user_input_iterator.next() {
            Some(arg3) => {
                if assign_flag_or_throw_error(arg3).is_err() {
                    return Err("Invalid flag");
                }
            }
            None => (),
        };
        if user_input_iterator.next().is_some() {
            return Err("Too many arguments");
        }
        return Ok(Config {
            query,
            file_name,
            insensitive,
            regex,
        });
    }
}

#[cfg(test)]
mod config_test;
