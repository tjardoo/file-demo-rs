use std::path::Path;

use crate::utils::path_without_base;

pub fn find_file_recursive(start_path: &Path, path: &Path, name: &str, depth: u32) {
    if depth == 0 {
        return;
    }

    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            find_file_recursive(start_path, &path, name, &depth - 1);
        } else if path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_lowercase()
            .contains(name.to_lowercase().as_str())
        {
            println!("{}", path_without_base(start_path, &path));
        }
    }
}
