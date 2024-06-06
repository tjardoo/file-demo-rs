use std::path::Path;

use crate::ansi_escape::TextStyling;
use crate::cli_parser::ListDirContentsType;
use crate::utils::print_without_start_path;

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
                ListDirContentsType::Dir => println!("{}", path.display().gray()),
                ListDirContentsType::Both => println!("{}", path.display().gray()),
                _ => {}
            };

            list_directory_contents_recursive(start_path, &path, r#type, depth - 1);
        } else {
            match r#type {
                ListDirContentsType::Dir => print_without_start_path(start_path, &path),
                ListDirContentsType::Both => print_without_start_path(start_path, &path),
                _ => {}
            };
        }
    }
}
