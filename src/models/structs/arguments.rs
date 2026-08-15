use std::path::Path;

//The arguments structure the rest of the search works on

pub struct Arguments {
    pub path: Option<PathBuf>,
    pub search_terms: Option<Vec<String>>,
    pub case_insensitive: bool,
}

impl Arguments {
    pub fn new() -> Arguments{
        return Arguments {
            path: None,
            search_terms: None,
            case_insensitive: false,
        };
    }

    pub fn get_path(&self) -> &PathBuf {
        return self.path.as_deref().unwrap_or_default();
    }

    pub fn get_search_terms(&self) -> &[String] {
        return self.search_terms.as_deref().unwrap_or_default();
    }

    pub fn get_case_insensitive(&self) -> bool {
        return self.case_insensitive;
    }
}
