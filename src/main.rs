use std::io::{self, Write};
use std::process::Command;

fn main() {
    loop {
        print!(">>> ");
        io::stdout();
        io::stdout().flush().expect("Failed to flush prompt");

        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read input");

        if line.trim() == "exit" {
            break;
        }
        else {
            parse_input(line.trim().to_string());
        }
    }
}

fn parse_input(line: String) {
    let split = line.split_whitespace();
    let words: Vec<&str> = split.collect();
    execute(words)
}

fn execute(cmd: Vec<&str>) {
    let output = Command::new(cmd[0]).args(&cmd[1..]).output().expect("Failed to execute");
    io::stdout().write_all(&output.stdout).unwrap();
}
