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
        let mut search_result: Vec<&str> = vec![];
        for line in self.content.lines() {
            if line.contains(&self.query) {
                search_result.push(line)
            }
        }
        return search_result;
    }

    pub fn insensitive_search(&self) -> Vec<&str> {
        let mut search_result: Vec<&str> = vec![];
        for line in self.content.lines() {
            if line.to_lowercase().contains(&self.query.to_lowercase()) {
                search_result.push(line)
            }
        }
        return search_result;
    }
}

#[cfg(test)]
mod search_engine_test;
