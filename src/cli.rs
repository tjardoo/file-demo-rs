use std::path::Path;

#[derive(Debug, PartialEq)]
pub enum Cli {
    List(ListDirContents),
    Find(FindFile),
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum ListDirContentsType {
    File,
    Dir,
    Both,
}

#[derive(Debug, PartialEq)]
pub struct ListDirContents {
    pub path: Box<Path>,
    pub depth: u32,
    pub r#type: ListDirContentsType,
}

#[derive(Debug, PartialEq)]
pub struct FindFile {
    pub path: Box<Path>,
    pub name: String,
    pub depth: u32,
}

impl Default for ListDirContents {
    fn default() -> Self {
        ListDirContents {
            path: std::env::current_dir().unwrap().into_boxed_path(),
            depth: 1,
            r#type: ListDirContentsType::Both,
        }
    }
}

impl Default for FindFile {
    fn default() -> Self {
        FindFile {
            path: std::env::current_dir().unwrap().into_boxed_path(),
            name: String::new(),
            depth: 3,
        }
    }
}
