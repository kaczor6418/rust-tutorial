use crate::Config;

#[test]
fn should_create_config_if_user_input_contains_two_arguments() {
    let valid_input = vec![String::from("query"), String::from("file.txt")];
    let config = Config::new(&valid_input);
    assert!(config.is_ok());
}

#[test]
fn should_create_config_with_insensitive_false_by_default() {
    let valid_input = vec![String::from("query"), String::from("file.txt")];
    let config = Config::new(&valid_input).unwrap();
    assert!(!config.insensitive);
}

#[test]
fn should_create_config_with_insensitive_flag_true() {
    let valid_input = vec![
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
    assert!(config.is_err());
}

#[test]
fn should_not_create_config_if_user_input_contains_less_then_two_arguments() {
    let invalid_input = vec![String::from("query")];
    let config = Config::new(&invalid_input);
    assert!(config.is_err());
}

#[test]
fn should_not_create_config_if_user_input_contains_more_then_three_arguments() {
    let invalid_input = vec![
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
        String::from("query"),
        String::from("file.txt"),
        String::from("incorrect flag"),
    ];
    let config = Config::new(&invalid_input);
    assert!(config.is_err());
}
