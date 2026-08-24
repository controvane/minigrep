use std::ffi::OsStr;
use std::fs::read_dir;
use std::path::{Path, PathBuf};

//recursive function that returns an threadable iterator
//If it is a file, it's path is added to the path list
//If it is a dir, we call walk over it and chain the result of that to what was found previously
//All on a Box cause we have no ideae how big it is
//Oh! and it ignores symlinks to avoid non ending loops
pub fn walk<'a>(path: &Path, file_types: &'a [&str]) -> Vec<PathBuf> {
    let (files, dirs): (Vec<PathBuf>, Vec<PathBuf>) = read_dir(path)
        .unwrap()
        .filter_map(Result::ok)
        .filter(|e| !matches!(e.file_type(), Ok(ft) if ft.is_symlink()))
        .map(|e| e.path())
        .partition(|p| p.is_file());

    let mut file_paths: Vec<PathBuf> = files
        .into_iter()
        .filter(|f| {
            file_types.is_empty()
                || f.extension().is_some_and(|ext| {
                    return file_types.iter().any(|t| ext == OsStr::new(t));
                })
        })
        .collect();

    for dir in dirs {
        file_paths.extend(walk(&dir, file_types));
    }

    return file_paths;
}
