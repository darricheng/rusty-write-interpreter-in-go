use crate::ast::{self, Program};
use crate::{lexer::Lexer, token::Token};

struct Parser {
    l: Lexer,
    current_token: Token,
    peek_token: Token,
}

impl Parser {
    fn new(l: Lexer) -> Parser {
        Parser {
            l,
            current_token: Token::new_placeholder(),
            peek_token: Token::new_placeholder(),
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    fn parse_program(&mut self) -> () {}
}
