use crate::search_engine::search_engine::SearchEngine;

fn first_line() -> &'static str {
    return "cat, dog, panda, same";
}

fn second_line() -> &'static str {
    return "elephant, tiger, crocodiles";
}

fn third_line() -> &'static str {
    return "bee, fly, eagle, same";
}

fn create_search_engine(search_value: &str) -> SearchEngine {
    let content = String::from(format!(
        "{}\n{}\n{}",
        first_line(),
        second_line(),
        third_line()
    ));
    let query = String::from(search_value);
    return SearchEngine::new(&query, &content);
}

mod case_sensitive {
    use crate::search_engine::search_engine::search_engine_test::{
        create_search_engine, first_line, third_line,
    };

    #[test]
    fn should_not_contain_any_result_for_zero_matches() {
        let search_value = "never";
        let search_engine = create_search_engine(search_value);
        let search_result = search_engine.search();
        assert!(search_result.is_empty());
    }

    #[test]
    fn should_contain_only_one_result_for_one_match() {
        let search_value = "dog";
        let search_engine = create_search_engine(search_value);
        let search_result = search_engine.search();
        assert_eq!(vec![first_line()], search_result);
    }

    #[test]
    fn should_contain_many_results_for_many_matches() {
        let search_value = "same";
        let search_engine = create_search_engine(search_value);
        let search_result = search_engine.search();
        assert_eq!(vec![first_line(), third_line()], search_result);
    }

    #[test]
    fn should_not_contain_any_result_for_insensitive_text() {
        let search_value = "TiGEr";
        let search_engine = create_search_engine(search_value);
        let search_result = search_engine.search();
        assert!(search_result.is_empty());
    }
}

mod case_insensitive {
    use crate::search_engine::search_engine::search_engine_test::{
        create_search_engine, first_line, third_line,
    };

    #[test]
    fn should_not_contain_any_result_for_zero_matches() {
        let search_value = "nEveR";
        let search_engine = create_search_engine(search_value);
        let search_result = search_engine.insensitive_search();
        assert!(search_result.is_empty());
    }

    #[test]
    fn should_contain_only_one_result_for_one_match() {
        let search_value = "DoG";
        let search_engine = create_search_engine(search_value);
        let search_result = search_engine.insensitive_search();
        assert_eq!(vec![first_line()], search_result);
    }

    #[test]
    fn should_contain_many_results_for_many_matches() {
        let search_value = "sAMe";
        let search_engine = create_search_engine(search_value);
        let search_result = search_engine.insensitive_search();
        assert_eq!(vec![first_line(), third_line()], search_result);
    }
}
