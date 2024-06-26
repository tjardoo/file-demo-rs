use std::path::Path;

use crate::ansi_escape::TextStyling;
use crate::cli::ListDirContentsType;
use crate::utils::path_without_base;

pub fn list_directory_contents_recursive(
    start_path: &Path,
    path: &Path,
    r#type: &ListDirContentsType,
    depth: u32,
) {
    if depth == 0 {
        return;
    }

    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            match r#type {
                ListDirContentsType::Dir => {
                    println!("{}", path_without_base(start_path, &path).gray())
                }
                ListDirContentsType::Both => {
                    println!("{}", path_without_base(start_path, &path).gray())
                }
                _ => {}
            };

            list_directory_contents_recursive(start_path, &path, r#type, depth - 1);
        } else {
            match r#type {
                ListDirContentsType::File => {
                    println!("{}", path_without_base(start_path, &path))
                }
                ListDirContentsType::Both => {
                    println!("{}", path_without_base(start_path, &path))
                }
                _ => {}
            };
        }
    }
}
