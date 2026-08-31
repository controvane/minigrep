use std::ffi::OsStr;
use std::fs::{File, read_dir};
use std::io::Read;
use std::path::{Path, PathBuf};

//recursive function that returns an threadable iterator
//If it is a file, it's path is added to the path list
//If it is a dir, we call walk over it and chain the result of that to what was found previously
//All on a Box cause we have no ideae how big it is
//Oh! and it ignores symlinks to avoid non ending loops
//Also, as we are looking on loop through a directory, this mechanism ignores binaries.
pub fn walk<'a>(
    path: &Path,
    file_types: &'a [&str],
) -> Box<dyn Iterator<Item = PathBuf> + Send + 'a> {
    let (files, dirs): (Vec<PathBuf>, Vec<PathBuf>) = read_dir(path)
        .unwrap()
        .filter_map(Result::ok)
        .filter(|e| !matches!(e.file_type(), Ok(ft) if ft.is_symlink()))
        .map(|e| e.path())
        .partition(|p| p.is_file());

    let files = files.into_iter().filter_map(|elem| {
        let mut file = File::open(&elem).ok()?;
        let mut head = [0u8; 8192];
        let n = file.read(&mut head).ok()?;
        if head[..n].contains(&0) {
            return None;
        } else {
            return Some(elem);
        }
    });

    return Box::new(
        files
            .into_iter()
            .filter(|f| {
                file_types.is_empty()
                    || f.extension().is_some_and(|ext| {
                        return file_types.iter().any(|t| ext == OsStr::new(t));
                    })
            })
            .chain(dirs.into_iter().flat_map(|d| walk(&d, file_types))),
    );
}
