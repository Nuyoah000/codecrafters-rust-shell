#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::Path;
use std::env;
use std::process::Command;

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

const BUILTINS: &[&str] = &["echo", "exit", "type"];

fn is_builtin(command: &str) -> bool {
    BUILTINS.contains(&command)
}

fn handle_command(command: &str) {
    if command.is_empty() {
        return;
    } 
    
    let mut parts = command.split_whitespace();
    let cmd_name = parts.next().unwrap();
    let args: Vec<&str> = parts.collect();

    if cmd_name == "echo" {
        let content = &command[5..];
        println!("{}", content);
    }
    else if cmd_name == "type" {
        let target = args.first().unwrap_or(&"");

        if is_builtin(target) {
            println!("{} is a shell builtin", target);
        } else {
            match find_executable_in_path(target) {
                Some(full_path) => println!("{} is {}", target, full_path),
                None => println!("{}: not found", target),
            }
        }
    }
    else {
        if let Some(full_path) = find_executable_in_path(cmd_name) {
            let _ = Command::new(full_path).args(args).spawn().and_then(|mut child| child.wait());
        } else {
            println!("{}: command not found", cmd_name);
        }
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
