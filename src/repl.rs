use std::io::{self, stdout, Write};

const PROMPT: &str = ">> ";

pub fn start() {
    let mut stdout = stdout();

    loop {
        print!("{}", PROMPT);
        stdout.flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input.");
        println!("Input: {}", input)
    }
}
