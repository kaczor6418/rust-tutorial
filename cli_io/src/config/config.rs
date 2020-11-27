pub struct Config {
    pub query: String,
    pub file_name: String,
}

impl Config {
    pub fn new(user_input: &Vec<String>) -> Result<Config, &str> {
        let error_message = Config::is_valid_input(user_input);
        if error_message.len() > 0 {
            return Err(error_message);
        }
        return Ok(Config {
            query: user_input[1].clone(),
            file_name: user_input[2].clone(),
        });
    }

    fn is_valid_input(user_input: &Vec<String>) -> &str {
        let mut result = "";
        let provided_arguments_count: usize = user_input.len() - 1;
        if provided_arguments_count < 2 {
            result =
                "Not enough arguments. You have to provide two arguments: query and file name.";
        } else if provided_arguments_count > 2 {
            result = "Too many arguments. You have to provide two arguments: query and file name!";
        }
        return result;
    }
}
