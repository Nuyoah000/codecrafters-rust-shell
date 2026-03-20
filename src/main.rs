#[allow(unused_imports)]
use std::io::{self, Write};

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

fn handle_command(command: &str) {
    if command.is_empty() {
        return;
    } else if command.starts_with("echo ") {
        let content = &command[5..];
        println!("{}", content);
    } else {
        println!("{}: command not found", command);
    }
}
