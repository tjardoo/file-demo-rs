mod ansi_escape;

use std::path::Path;

use ansi_escape::TextStyling;

enum Cli {
    List { directory: Box<Path>, depth: u32 },
    Find { directory: Box<Path>, name: String },
}

fn parse_args(args: &[String]) -> Cli {
    if args.len() < 2 {
        panic!("Please provide a subcommand");
    }

    let subcommand = args.get(1).unwrap();

    match subcommand.as_str() {
        "list" => {
            if args.len() < 3 {
                panic!("Please provide a 'directory' to list");
            } else if args.len() < 4 {
                panic!("Please provide a 'depth' to list");
            }

            let directory = Path::new(args.get(2).unwrap())
                .to_path_buf()
                .into_boxed_path();
            let depth = args.get(3).unwrap().parse::<u32>().unwrap();

            Cli::List { directory, depth }
        }
        "find" => {
            if args.len() < 3 {
                panic!("Please provide a 'directory' to find");
            } else if args.len() < 4 {
                panic!("Please provide a 'name' to find");
            }

            let directory = Path::new(args.get(2).unwrap())
                .to_path_buf()
                .into_boxed_path();
            let name = args.get(3).unwrap().to_string();

            Cli::Find { directory, name }
        }
        _ => panic!("Unknown subcommand"),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let cli = parse_args(&args);

    match cli {
        Cli::List { directory, depth } => {
            println!("{}", "Listing directory contents".blue().bold());
            println!("{}: {}\n", "Directory".gray(), directory.display().bold());

            list_directory_contents_recursive(&directory, &directory, depth);
        }
        Cli::Find { directory, name } => {
            println!("{} {}", "Finding file:".gray(), name.clone().blue().bold());
            println!("{}: {}\n", "Directory".gray(), directory.display().bold());

            find_file_recursive(&directory, &directory, &name);
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
