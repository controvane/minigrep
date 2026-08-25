use regex::Regex;

//This whole enum is to support quicker case insensitive search_term
//it turns the search term into a regex in case the case should #![no_std]
//be considered.
pub enum SearchMode {
    Sensitive(String),
    Insensitive(Regex),
}

impl SearchMode {
    pub fn new(search_term: String, case_insensitive: bool) -> Self {
        if case_insensitive {
            let escaped = regex::escape(&search_term);
            let prepped_term = Regex::new(&format!("(?i){}", escaped)).unwrap();
            return SearchMode::Insensitive(prepped_term);
        }
        return SearchMode::Sensitive(search_term);
    }

    pub fn matches(&self, line: &str) -> bool {
        match self {
            SearchMode::Sensitive(value) => line.contains(value),
            SearchMode::Insensitive(value) => value.is_match(line),
        }
    }
}
