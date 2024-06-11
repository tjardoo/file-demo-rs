use cli::Cli;
use container::init_container;
use state_machine::{Done, Filling, StateMachine};
use utils::print_general_cli_info;

mod ansi_escape;
mod cli;
mod cli_parser;
mod command;
mod container;
mod macros;
mod state_machine;
mod utils;

fn main() {
    init_container();

    let args: Vec<String> = std::env::args().collect();

    let cli = cli_parser::parse_args(&args);

    let in_waiting = StateMachine::new(0);
    let in_filling = StateMachine::<Filling>::from(in_waiting);
    let _in_done = StateMachine::<Done>::from(in_filling);

    print_general_cli_info(&cli);

    match cli {
        Cli::List(list_dir_contents) => {
            command::list::list_directory_contents_recursive(
                list_dir_contents.path.as_ref(),
                list_dir_contents.path.as_ref(),
                &list_dir_contents.r#type,
                list_dir_contents.depth,
            );
        }
        Cli::Find(find_file) => {
            command::find::find_file_recursive(
                find_file.path.as_ref(),
                find_file.path.as_ref(),
                &find_file.name,
                find_file.depth,
            );
        }
    }
}
