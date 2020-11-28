pub struct Config {
    pub query: String,
    pub file_name: String,
    pub insensitive: bool,
}

impl Config {
    const INSENSITIVE: &'static str = "--insensitive";
    pub fn new(user_input: &Vec<String>) -> Result<Config, &str> {
        let error_message = Config::is_valid_input(user_input);
        if error_message.len() > 0 {
            return Err(error_message);
        }
        return Ok(Config {
            query: user_input[1].clone(),
            file_name: user_input[2].clone(),
            insensitive: user_input.contains(&String::from(Config::INSENSITIVE)),
        });
    }

    fn is_valid_input(user_input: &Vec<String>) -> &str {
        let mut result = "";
        let provided_arguments_count: usize = user_input.len();
        if provided_arguments_count < 3 {
            result =
                "Not enough arguments. You have to provide two arguments: query and file name.";
        } else if provided_arguments_count == 4 && user_input[3] != Config::INSENSITIVE {
            result = "Invalid flag";
        } else if provided_arguments_count > 4 {
            result = "Too many arguments. You have to provide two arguments: query and file name!";
        }
        return result;
    }
}

#[cfg(test)]
mod config_tests {
    use crate::Config;

    #[test]
    fn should_create_config_if_user_input_contains_three_arguments() {
        let valid_input = vec![
            String::from("crate"),
            String::from("query"),
            String::from("file.txt"),
        ];
        let config = Config::new(&valid_input);
        assert!(config.is_ok());
    }

    #[test]
    fn should_create_config_with_insensitive_false_by_default() {
        let valid_input = vec![
            String::from("crate"),
            String::from("query"),
            String::from("file.txt"),
        ];
        let config = Config::new(&valid_input).unwrap();
        assert!(!config.insensitive);
    }

    #[test]
    fn should_create_config_with_insensitive_flag_true() {
        let valid_input = vec![
            String::from("crate"),
            String::from("query"),
            String::from("file.txt"),
            String::from(Config::INSENSITIVE),
        ];
        let config = Config::new(&valid_input).unwrap();
        assert!(config.insensitive);
    }

    #[test]
    fn should_not_create_config_if_user_input_is_empty() {
        let invalid_input = vec![];
        let config = Config::new(&invalid_input);
        println!("Result: {}", config.is_err());
        assert!(config.is_err());
    }

    #[test]
    fn should_not_create_config_if_user_input_contains_less_then_three_arguments() {
        let invalid_input = vec![String::from("query")];
        let config = Config::new(&invalid_input);
        assert!(config.is_err());
    }

    #[test]
    fn should_not_create_config_if_user_input_contains_more_then_four_arguments() {
        let invalid_input = vec![
            String::from("crate"),
            String::from("query"),
            String::from("file.txt"),
            String::from(Config::INSENSITIVE),
            String::from("more"),
        ];
        let config = Config::new(&invalid_input);
        assert!(config.is_err());
    }

    #[test]
    fn should_not_create_config_if_user_input_flag_is_incorrect() {
        let invalid_input = vec![
            String::from("crate"),
            String::from("query"),
            String::from("file.txt"),
            String::from("incorrect flag"),
        ];
        let config = Config::new(&invalid_input);
        assert!(config.is_err());
    }
}
