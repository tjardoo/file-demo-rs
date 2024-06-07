#[macro_export]
macro_rules! handle_cli_list_argument {
    ($arg:expr, $list_dir_contents:expr) => {
        if let Some((key, value)) = $arg.split_once('=') {
            match key {
                "--directory" => {
                    $list_dir_contents.directory = Path::new(value).to_path_buf().into_boxed_path();
                }
                "--type" => {
                    $list_dir_contents.r#type = match value {
                        "file" => ListDirContentsType::File,
                        "dir" => ListDirContentsType::Dir,
                        _ => ListDirContentsType::Both,
                    };
                }
                "--depth" => {
                    $list_dir_contents.depth = value.parse().unwrap();
                }
                _ if key.starts_with("--") => {
                    panic!("Invalid argument: {}", $arg);
                }
                _ => {}
            }
        } else if $arg.starts_with("--") {
            panic!("Invalid argument: {}", $arg);
        }
    };
}

#[macro_export]
macro_rules! handle_cli_find_argument {
    ($arg:expr, $list_dir_contents:expr) => {
        if let Some((key, value)) = $arg.split_once('=') {
            match key {
                "--directory" => {
                    $list_dir_contents.directory = Path::new(value).to_path_buf().into_boxed_path();
                }
                "--name" => {
                    $list_dir_contents.name = value.to_string();
                }
                "--depth" => {
                    $list_dir_contents.depth = value.parse().unwrap();
                }
                _ if key.starts_with("--") => {
                    panic!("Invalid argument: {}", $arg);
                }
                _ => {}
            }
        } else if $arg.starts_with("--") {
            panic!("Invalid argument: {}", $arg);
        }
    };
}
