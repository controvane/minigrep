//Just the possible contents of the input
//Used by the hashmap

#[derive(Debug)]
pub enum FlagValue {
    Path(String),
    SearchTerms(Vec<String>),
    CaseSensitive(bool),
}
