mod ansi_escape;

use std::path::Path;

use ansi_escape::TextStyling;

enum Cli {
    List(u32),
    Find(String),
}

fn parse_args(args: &[String]) -> Cli {
    if args.len() < 2 {
        panic!("Please provide a subcommand");
    }

    let subcommand = args.get(1).unwrap();

    match subcommand.as_str() {
        "list" => {
            if args.len() < 3 {
                panic!("Please provide a 'depth' to list");
            }

            let depth = args.get(2).unwrap().parse::<u32>().unwrap();

            Cli::List(depth)
        }
        "find" => {
            if args.len() < 3 {
                panic!("Please provide a 'file name' to find");
            }

            let name = args.get(2).unwrap().to_string();

            Cli::Find(name)
        }
        _ => panic!("Unknown subcommand"),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let cli = parse_args(&args);

    let current_dir = std::env::current_dir().unwrap();

    match cli {
        Cli::List(depth) => {
            println!("{}", "Listing directory contents".blue().bold());
            println!(
                "{}: {}\n",
                "Current directory".gray(),
                current_dir.display().bold()
            );

            list_directory_contents_recursive(&current_dir, &current_dir, depth);
        }
        Cli::Find(name) => {
            println!("{} {}", "Finding file:".gray(), name.clone().blue().bold());
            println!(
                "{}: {}\n",
                "Current directory".gray(),
                current_dir.display().bold()
            );

            find_file_recursive(&current_dir, &current_dir, &name);
        }
    }
}

fn list_directory_contents_recursive(start_path: &Path, path: &Path, depth: u32) {
    if depth == 0 {
        return;
    }

    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            println!("{}", path.display().gray());

            list_directory_contents_recursive(start_path, &path, depth - 1);
        } else {
            print_without_current_path(start_path, &path);
        }
    }
}

fn find_file_recursive(start_path: &Path, path: &Path, name: &str) {
    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            find_file_recursive(start_path, &path, name);
        } else if path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_lowercase()
            .contains(name.to_lowercase().as_str())
        {
            print_without_current_path(start_path, &path);
        }
    }
}

fn print_without_current_path(start_path: &Path, path: &Path) {
    let path = path.strip_prefix(start_path).unwrap();

    println!("{}", path.display());
}
