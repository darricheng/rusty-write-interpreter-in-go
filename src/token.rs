use std::str::{self, from_utf8};

pub type TokenType<'a> = &'a str;

#[derive(Debug)]
pub struct Token<'a> {
    pub token_type: TokenType<'a>,
    pub literal: String,
}

impl<'a> Token<'_> {
    pub fn new(token_type: TokenType, byte: u8) -> Token {
        let literal: String = from_utf8(&[byte]).unwrap().to_string();
        Token {
            token_type,
            literal,
        }
    }
}

pub const ILLEGAL: &str = "ILLEGAL";
pub const EOF: &str = "EOF";

// Identifiers + literals
pub const IDENT: &str = "IDENT";
pub const INT: &str = "INT";

pub const ASSIGN: &str = "=";
pub const PLUS: &str = "+";

// Delimiters
pub const COMMA: &str = ",";
pub const SEMICOLON: &str = ";";

pub const LPAREN: &str = "(";
pub const RPAREN: &str = ")";
pub const LBRACE: &str = "{";
pub const RBRACE: &str = "}";

// Keywords
pub const FUNCTION: &str = "FUNCTION";
pub const LET: &str = "LET";
