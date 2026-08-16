use std::path::PathBuf;

//Just the possible contents of the input
//Used by the hashmap

#[derive(Debug)]
pub enum FlagValue {
    Path(PathBuf),
    SearchTerms(Vec<String>),
    CaseSensitive(bool),
}
