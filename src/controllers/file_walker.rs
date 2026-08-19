use std::fs::read_dir;
use std::path::{Path, PathBuf};

//recursive function that returns an threadable iterator
//If it is a file, it's path is added to the path list
//If it is a dir, we call walk over it and chain the result of that to what was found previously
//All on a Box cause we have no ideae how big it is
//Oh! and it ignores symlinks to avoid non ending loops
pub fn walk(path: &Path) -> Box<dyn Iterator<Item = PathBuf> + Send> {
    let (files, dirs): (Vec<PathBuf>, Vec<PathBuf>) = read_dir(path)
        .unwrap()
        .filter_map(Result::ok)
        .filter(|e| !matches!(e.file_type(), Ok(ft) if ft.is_symlink()))
        .map(|e| e.path())
        .partition(|p| p.is_file());

    return Box::new(
        files
            .into_iter()
            .chain(dirs.into_iter().flat_map(|d| walk(&d))),
    );
}
