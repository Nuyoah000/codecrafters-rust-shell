#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    shell_loop();
}

fn shell_loop() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let command = read_input_and_trim();

        if command.is_empty() {
            continue;
        }

        if command == "exit" {
            break;
        }

        // handle_invalid_command(&command);
        handle_command(&command);
    }
}

fn read_input_and_trim() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

// fn handle_invalid_command(command: &str) {
//     println!("{}: command not found", command);
// }

fn handle_command(command: &str) {
    if command.starts_with("echo ") {
        let content = &command[5..];
        println!("{}", content);
    } else {
        println!("{}: command not found", command);
    }
}