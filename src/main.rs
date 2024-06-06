use cli_parser::Cli;
use utils::print_general_cli_info;

mod ansi_escape;
mod cli_parser;
mod command;
mod utils;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let cli = cli_parser::parse_args(&args);

    print_general_cli_info(&cli);

    match cli {
        Cli::List(list_dir_contents) => {
            command::list::list_directory_contents_recursive(
                list_dir_contents.directory.as_ref(),
                list_dir_contents.directory.as_ref(),
                &list_dir_contents.r#type,
                list_dir_contents.depth,
            );
        }
        Cli::Find(find_file) => {
            command::find::find_file_recursive(
                find_file.directory.as_ref(),
                find_file.directory.as_ref(),
                &find_file.name,
                find_file.depth,
            );
        }
    }
}
