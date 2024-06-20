#[macro_export]
macro_rules! handle_cli_list_argument {
    ($arg:expr, $list_dir_contents:expr) => {
        if let Some((key, value)) = $arg.split_once('=') {
            match key {
                "--path" | "-p" => {
                    $list_dir_contents.path = Path::new(value).to_path_buf().into_boxed_path();
                }
                "--type" | "-t" => {
                    $list_dir_contents.r#type = match value {
                        "file" => $crate::cli::ListDirContentsType::File,
                        "dir" => $crate::cli::ListDirContentsType::Dir,
                        _ => $crate::cli::ListDirContentsType::Both,
                    };
                }
                "--depth" | "-d" => {
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
                "--path" | "-p" => {
                    $list_dir_contents.path = Path::new(value).to_path_buf().into_boxed_path();
                }
                "--name" | "-n" => {
                    $list_dir_contents.name = value.to_string();
                }
                "--depth" | "-d" => {
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
macro_rules! handle_cli_users_argument {
    ($arg:expr, $user_filter_input:expr) => {
        if let Some((key, value)) = $arg.split_once('=') {
            match key {
                "--name" | "-n" => {
                    $user_filter_input.name = Some(value.to_string());
                }
                "--min" => {
                    $user_filter_input.min_age = Some(value.parse().unwrap());
                }
                "--max" => {
                    $user_filter_input.max_age = Some(value.parse().unwrap());
                }
                "--active" | "-a" => {
                    $user_filter_input.is_active = Some(value.parse().unwrap());
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
