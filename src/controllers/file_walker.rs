use std::fs::read_dir;
use std::path::{Path, PathBuf};

pub fn walk(path: &Path) -> Box<dyn Iterator<Item = PathBuf>> {
    let (files, dirs): (Vec<PathBuf>, Vec<PathBuf>) = read_dir(path)
        .unwrap()
        .filter_map(Result::ok)
        .map(|e| e.path())
        .partition(|p| p.is_file());

    return Box::new(
        files
            .into_iter()
            .chain(dirs.into_iter().flat_map(|d| walk(&d))),
    );
}
