use std::io::{self, Write};

fn main() {
    loop {
        print!(">>> ");
        io::stdout();
        let _ = io::stdout().flush();

        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read input");

        if line.trim() == "exit" {
            break;
        }
        else {
            println!("{}", line);
        }
    }
}

