use crate::{lexer::Lexer, token::TokenType};
use std::io::{self, stdout, Write};

const PROMPT: &str = ">> ";

pub fn start() {
    let mut stdout = stdout();

    println!("Rusty Monkey Programming Languague v0.1.0");

    loop {
        print!("{}", PROMPT);
        stdout.flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input.");

        let mut l = Lexer::new(input);

        loop {
            let tok = l.next_token();
            if tok.token_type == TokenType::EOF {
                break;
            }
            println!("{:?}", tok);
        }
    }
}
