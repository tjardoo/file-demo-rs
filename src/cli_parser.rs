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
    pub directory: Box<Path>,
    pub depth: u32,
    pub r#type: ListDirContentsType,
}

#[derive(Debug, PartialEq)]
pub struct FindFile {
    pub directory: Box<Path>,
    pub name: String,
    pub depth: u32,
}

pub fn parse_args(args: &[String]) -> Cli {
    let command = args.get(1).unwrap_or_else(|| {
        panic!("Please provide a command");
    });

    let cli = match command.as_str() {
        "list" => parse_list_command(args),
        "find" => parse_find_command(args),
        _ => panic!("Invalid command"),
    };

    cli
}

fn parse_list_command(args: &[String]) -> Cli {
    let mut list_dir_contents = ListDirContents {
        directory: std::env::current_dir().unwrap().into_boxed_path(),
        depth: 1,
        r#type: ListDirContentsType::Both,
    };

    for arg in args.iter().skip(2) {
        if arg.starts_with("--directory=") {
            let directory = arg.split('=').last().unwrap();

            list_dir_contents.directory = Path::new(directory).to_path_buf().into_boxed_path();
        } else if arg.starts_with("--type=") {
            let r#type = arg.split('=').last().unwrap();

            list_dir_contents.r#type = match r#type {
                "file" => ListDirContentsType::File,
                "dir" => ListDirContentsType::Dir,
                _ => ListDirContentsType::Both,
            };
        } else if arg.starts_with("--depth=") {
            let depth = arg.split('=').last().unwrap();

            list_dir_contents.depth = depth.parse().unwrap();
        } else if arg.starts_with("--") {
            panic!("Invalid argument: {}", arg);
        }
    }

    Cli::List(list_dir_contents)
}

fn parse_find_command(args: &[String]) -> Cli {
    let mut find_file = FindFile {
        directory: std::env::current_dir().unwrap().into_boxed_path(),
        name: String::new(),
        depth: 3,
    };

    for arg in args.iter().skip(2) {
        if arg.starts_with("--directory=") {
            let directory = arg.split('=').last().unwrap();

            find_file.directory = Path::new(directory).to_path_buf().into_boxed_path();
        } else if arg.starts_with("--name=") {
            let name = arg.split('=').last().unwrap();

            find_file.name = name.to_string();
        } else if arg.starts_with("--depth=") {
            let depth = arg.split('=').last().unwrap();

            find_file.depth = depth.parse().unwrap();
        } else if arg.starts_with("--") {
            panic!("Invalid argument: {}", arg);
        }
    }

    if find_file.name.is_empty() {
        panic!("Please provide a name to search for");
    }

    Cli::Find(find_file)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_list_command() {
        let args = vec![
            "N/A".to_string(),
            "list".to_string(),
            "--directory=/var/www".to_string(),
            "--depth=2".to_string(),
            "--type=file".to_string(),
        ];

        let cli = parse_list_command(&args);

        assert_eq!(
            cli,
            Cli::List(ListDirContents {
                directory: Path::new("/var/www").to_path_buf().into_boxed_path(),
                depth: 2,
                r#type: ListDirContentsType::File,
            })
        );
    }

    #[test]
    fn test_parse_list_command_default() {
        let args = vec!["N/A".to_string(), "list".to_string()];

        let cli = parse_list_command(&args);

        assert_eq!(
            cli,
            Cli::List(ListDirContents {
                directory: std::env::current_dir().unwrap().into_boxed_path(),
                depth: 1,
                r#type: ListDirContentsType::Both,
            })
        );
    }

    #[test]
    fn test_parse_find_command() {
        let args = vec![
            "N/A".to_string(),
            "find".to_string(),
            "--directory=/var/www".to_string(),
            "--name=example.txt".to_string(),
            "--depth=1".to_string(),
        ];

        let cli = parse_find_command(&args);

        assert_eq!(
            cli,
            Cli::Find(FindFile {
                directory: Path::new("/var/www").to_path_buf().into_boxed_path(),
                name: "example.txt".to_string(),
                depth: 1,
            })
        );
    }

    #[test]
    fn test_parse_find_command_default() {
        let args = vec![
            "N/A".to_string(),
            "find".to_string(),
            "--name=example.txt".to_string(),
        ];

        let cli = parse_find_command(&args);

        assert_eq!(
            cli,
            Cli::Find(FindFile {
                directory: std::env::current_dir().unwrap().into_boxed_path(),
                name: "example.txt".to_string(),
                depth: 3,
            })
        );
    }

    #[test]
    #[should_panic]
    fn test_parse_find_command_without_name() {
        let args = vec!["N/A".to_string(), "find".to_string()];

        parse_find_command(&args);
    }
}