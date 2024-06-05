enum Cli {
    List(u32),
    Find(String),
}

fn parse_args(args: &Vec<String>) -> Cli {
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

    println!("The current directory is '{}'", current_dir.display());

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
            print_gray(path.display());

            list_directory_contents_recursive(&path, depth - 1);
        } else {
            print_green(path.display());
        }
    }
}

fn find_file_recursive(path: &std::path::Path, name: &str) {
    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            find_file_recursive(&path, name);
        } else {
            if path.file_name().unwrap() == name {
                print_green(path.display());
            }
        }
    }
}

#[allow(dead_code)]
fn print_gray<T: std::fmt::Display>(text: T) {
    println!("\x1b[90m{}\x1b[0m", text);
}

#[allow(dead_code)]
fn print_red<T: std::fmt::Display>(text: T) {
    println!("\x1b[91m{}\x1b[0m", text);
}

#[allow(dead_code)]
fn print_green<T: std::fmt::Display>(text: T) {
    println!("\x1b[92m{}\x1b[0m", text);
}

#[allow(dead_code)]
fn print_yellow<T: std::fmt::Display>(text: T) {
    println!("\x1b[93m{}\x1b[0m", text);
}

#[allow(dead_code)]
fn print_blue(text: &dyn std::fmt::Display) {
    println!("\x1b[94m{}\x1b[0m", text);
}
