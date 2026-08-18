use std::path::PathBuf;

pub enum InputSource {
    Normal(PathBuf),
    Stdin,
}
