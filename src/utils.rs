use std::path::Path;

use crate::ansi_escape::TextStyling;
use crate::cli::{Cli, ListDirContentsType};

pub fn print_general_cli_info(cli: &Cli) {
    match cli {
        Cli::List(list_dir_contents) => {
            match list_dir_contents.r#type {
                ListDirContentsType::File => {
                    println!("{}", "Listing files".blue().bold());
                }
                ListDirContentsType::Dir => {
                    println!("{}", "Listing directories".blue().bold());
                }
                ListDirContentsType::Both => {
                    println!("{}", "Listing files and directories".blue().bold());
                }
            };

            println!(
                "{} {}\n",
                "Path:".gray(),
                list_dir_contents.directory.display().bold()
            );
        }
        Cli::Find(find_file) => {
            println!(
                "{} {}",
                "Finding files containing the name:".blue(),
                find_file.name.clone().blue().bold()
            );

            println!(
                "{} {}\n",
                "Path:".gray(),
                find_file.directory.display().bold()
            );
        }
    };
}

pub fn print_without_start_path(start_path: &Path, path: &Path) {
    let path = path.strip_prefix(start_path).unwrap();

    println!("{}", path.display());
}
