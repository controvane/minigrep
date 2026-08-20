use crate::models::enums::pos_path::InputSource;
use std::env;
use std::process;

//The arguments structure the rest of the search works on

pub struct Arguments {
    pub path: InputSource,
    pub or_search_terms: Option<Vec<String>>,
    pub and_search_terms: Option<Vec<String>>,
    pub ex_search_terms: Option<Vec<String>>,
    pub case_insensitive: bool,
    pub before_lines: u16,
    pub after_lines: u16,
}

impl Arguments {
    pub fn new() -> Arguments {
        return Arguments {
            path: InputSource::Normal(match env::current_dir() {
                Ok(value) => value,
                Err(_) => {
                    eprintln!("Really? No pwd? How did this happen?");
                    process::exit(1);
                }
            }),
            or_search_terms: None,
            and_search_terms: None,
            ex_search_terms: None,
            case_insensitive: false,
            before_lines: 0,
            after_lines: 0,
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

    pub fn get_before_lines(&self) -> u16 {
        return self.before_lines;
    }

    pub fn get_after_lines(&self) -> u16 {
        return self.after_lines;
    }
}
