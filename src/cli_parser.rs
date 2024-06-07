use std::path::Path;

use crate::{
    cli::{Cli, FindFile, ListDirContents},
    handle_cli_find_argument, handle_cli_list_argument,
};

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
    let mut list_dir_contents = ListDirContents::default();

    for arg in args.iter().skip(2) {
        handle_cli_list_argument!(arg, list_dir_contents);
    }

    Cli::List(list_dir_contents)
}

fn parse_find_command(args: &[String]) -> Cli {
    let mut find_file = FindFile::default();

    for arg in args.iter().skip(2) {
        handle_cli_find_argument!(arg, find_file);
    }

    if find_file.name.is_empty() {
        panic!("Please provide a name to search for");
    }

    Cli::Find(find_file)
}

#[cfg(test)]
mod tests {
    use crate::cli::ListDirContentsType;

    use super::*;

    #[test]
    fn test_parse_list_command() {
        let args = vec![
            "N/A".to_string(),
            "list".to_string(),
            "--path=/var/www".to_string(),
            "--depth=2".to_string(),
            "--type=file".to_string(),
        ];

        let cli = parse_list_command(&args);

        assert_eq!(
            cli,
            Cli::List(ListDirContents {
                path: Path::new("/var/www").to_path_buf().into_boxed_path(),
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
                path: std::env::current_dir().unwrap().into_boxed_path(),
                depth: 1,
                r#type: ListDirContentsType::Both,
            })
        );
    }

    #[test]
    fn test_parse_list_command_shortcodes() {
        let args = vec![
            "N/A".to_string(),
            "list".to_string(),
            "-p=/var/www".to_string(),
            "-d=2".to_string(),
            "-t=file".to_string(),
        ];

        let cli = parse_list_command(&args);

        assert_eq!(
            cli,
            Cli::List(ListDirContents {
                path: Path::new("/var/www").to_path_buf().into_boxed_path(),
                depth: 2,
                r#type: ListDirContentsType::File,
            })
        );
    }

    #[test]
    fn test_parse_find_command() {
        let args = vec![
            "N/A".to_string(),
            "find".to_string(),
            "--path=/var/www".to_string(),
            "--name=example.txt".to_string(),
            "--depth=1".to_string(),
        ];

        let cli = parse_find_command(&args);

        assert_eq!(
            cli,
            Cli::Find(FindFile {
                path: Path::new("/var/www").to_path_buf().into_boxed_path(),
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
                path: std::env::current_dir().unwrap().into_boxed_path(),
                name: "example.txt".to_string(),
                depth: 3,
            })
        );
    }

    #[test]
    fn test_parse_find_command_shortcodes() {
        let args = vec![
            "N/A".to_string(),
            "find".to_string(),
            "-p=/var/www".to_string(),
            "-n=example.txt".to_string(),
            "-d=1".to_string(),
        ];

        let cli = parse_find_command(&args);

        assert_eq!(
            cli,
            Cli::Find(FindFile {
                path: Path::new("/var/www").to_path_buf().into_boxed_path(),
                name: "example.txt".to_string(),
                depth: 1,
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
