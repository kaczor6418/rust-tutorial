pub struct Config {
    pub query: String,
    pub file_name: String,
    pub insensitive: bool,
}

impl Config {
    pub const INSENSITIVE: &'static str = "--insensitive";
    pub fn new(user_input: &Vec<String>) -> Result<Config, &str> {
        let error_message = Config::is_valid_input(user_input);
        if error_message.len() > 0 {
            return Err(error_message);
        }
        return Ok(Config {
            query: user_input[0].clone(),
            file_name: user_input[1].clone(),
            insensitive: user_input.contains(&String::from(Config::INSENSITIVE)),
        });
    }

    fn is_valid_input(user_input: &Vec<String>) -> &str {
        let mut result = "";
        let provided_arguments_count: usize = user_input.len();
        if provided_arguments_count < 2 {
            result =
                "Not enough arguments. You have to provide at least two arguments: query and file name.";
        } else if provided_arguments_count == 3 && user_input[2] != Config::INSENSITIVE {
            result = "Invalid flag";
        } else if provided_arguments_count > 3 {
            result = "Too many arguments. You have to provide maximum three arguments: query, file name, flag.";
        }
        return result;
    }
}

#[cfg(test)]
mod config_test;
