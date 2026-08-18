use crate::models::enums::pos_path::InputSource;
use std::env;

//The arguments structure the rest of the search works on

pub struct Arguments {
    pub path: InputSource,
    pub or_search_terms: Option<Vec<String>>,
    pub and_search_terms: Option<Vec<String>>,
    pub ex_search_terms: Option<Vec<String>>,
    pub case_insensitive: bool,
}

impl Arguments {
    pub fn new() -> Arguments {
        return Arguments {
            path: InputSource::Normal(
                env::current_dir().expect("Really? How did this happened? No cwd?"),
            ),
            or_search_terms: None,
            and_search_terms: None,
            ex_search_terms: None,
            case_insensitive: false,
        };
    }

    pub fn get_path(&self) -> &InputSource {
        return &self.path;
    }

    pub fn get_search_terms_or(&self) -> &[String] {
        return self.or_search_terms.as_deref().unwrap_or_default();
    }

    pub fn get_search_terms_and(&self) -> &[String] {
        return self.and_search_terms.as_deref().unwrap_or_default();
    }

    pub fn get_search_terms_ex(&self) -> &[String] {
        return self.ex_search_terms.as_deref().unwrap_or_default();
    }

    pub fn get_case_insensitive(&self) -> bool {
        return self.case_insensitive;
    }
}
