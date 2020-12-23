use regex::Regex;

pub struct SearchEngine {
    query: String,
    content: String,
}

impl SearchEngine {
    pub fn new(query: &String, content: &String) -> SearchEngine {
        return SearchEngine {
            query: query.clone(),
            content: content.clone(),
        };
    }

    pub fn search(&self) -> Vec<&str> {
        return self
            .content
            .lines()
            .filter(|line| line.contains(&self.query))
            .collect();
    }

    pub fn insensitive_search(&self) -> Vec<&str> {
        return self
            .content
            .lines()
            .filter(|line| line.to_lowercase().contains(&self.query.to_lowercase()))
            .collect();
    }

    pub fn regex_search(&self, insensitive: bool) -> Vec<&str> {
        let insensitive_flag = match insensitive {
            true => "(?i)",
            false => "",
        };
        let expression = format!("{}{}", insensitive_flag, self.query);
        let re = Regex::new(&expression).unwrap();
        return self
            .content
            .lines()
            .filter(|line| re.is_match(line))
            .collect();
    }
}

#[cfg(test)]
mod search_engine_test;
