use dirs;
use std::path::{Path, PathBuf};

//From: https://stackoverflow.com/questions/54267608/expand-tilde-in-rust-path-idiomatically
pub fn expand_if_contains_tilde<TPath: AsRef<Path>>(input: TPath) -> Option<PathBuf> {
    let path = input.as_ref();
    if path.starts_with("~") == false {
        return Some(path.to_path_buf());
    }
    if path == Path::new("~") {
        return dirs::home_dir();
    }

    dirs::home_dir().map(|mut home_path| {
        if home_path == Path::new("/") {
            // Corner case: `h` root directory;
            // don't prepend extra `/`, just drop the tilde.
            path.strip_prefix("~").unwrap().to_path_buf()
        } else {
            home_path.push(path.strip_prefix("~/").unwrap());
            home_path
        }
    })
}
