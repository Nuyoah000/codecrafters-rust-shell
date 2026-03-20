use std::env;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::Path;

const BUILTINS: &[&str] = &["echo", "exit", "type"];

fn main() {
    shell_loop();
}

fn shell_loop() {
    loop {
        print_prompt();

        let command = read_input_and_trim();

        if command == "exit" {
            break;
        }

        handle_command(&command);
    }
}

fn print_prompt() {
    print!("$ ");
    io::stdout().flush().unwrap();
}

fn read_input_and_trim() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn is_builtin(command: &str) -> bool {
    BUILTINS.contains(&command)
}

fn handle_command(command: &str) {
    if command.is_empty() {
        return;
    } else if command.starts_with("echo ") {
        let content = &command[5..];
        println!("{}", content);
    } else if command.starts_with("type ") {
        let target = &command[5..];
        if is_builtin(target) {
            println!("{} is a shell builtin", target);
        } else {
            match find_executable_in_path(target) {
                Some(full_path) => println!("{} is {}", target, full_path),
                None => println!("{}: not found", target),
            }
        }
    } else {
        println!("{}: command not found", command);
    }
}

fn find_executable_in_path(command: &str) -> Option<String> {
    let path_env = env::var("PATH").ok()?;

    for dir in env::split_paths(&path_env) {
        let full_path = dir.join(command);

        if !full_path.is_file() {
            continue;
        }

        if is_executable(&full_path) {
            return full_path.to_str().map(|s| s.to_string());
        }
    }

    None
}

fn is_executable(path: &Path) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        path.metadata()
            .map(|m| m.permissions().mode() & 0o111 != 0)
            .unwrap_or(false)
    }

    #[cfg(windows)]
    {
        path.exists()
    }
}
