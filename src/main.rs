mod ansi_escape;

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

    println!(
        "{} {}\n",
        "Current directory:".gray(),
        current_dir.display().bold(),
    );

    match cli {
        Cli::List(depth) => list_directory_contents_recursive(&current_dir, depth),
        Cli::Find(name) => find_file_recursive(&current_dir, &name),
    }
}

fn list_directory_contents_recursive(path: &std::path::Path, depth: u32) {
    if depth == 0 {
        return;
    }

    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            println!("{}", path.display().gray());

            list_directory_contents_recursive(&path, depth - 1);
        } else {
            println!("{}", path.display().white());
        }
    }
}

fn find_file_recursive(path: &std::path::Path, name: &str) {
    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            find_file_recursive(&path, name);
        } else if path.file_name().unwrap() == name {
            println!("{}", path.display().blue());
        }
    }
}
