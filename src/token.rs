use std::str::{self, from_utf8};

#[derive(Debug, PartialEq)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    // Identifiers + literals
    IDENT,
    INT,
    // Operators
    ASSIGN,
    PLUS,
    // Delimiters
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    // Keywords
    FUNCTION,
    LET,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new_from_str(token_type: TokenType, str: &str) -> Token {
        let literal = str.to_string();
        Token {
            token_type,
            literal,
        }
    }
    pub fn new_from_byte(token_type: TokenType, byte: u8) -> Token {
        let literal: String = from_utf8(&[byte]).unwrap().to_string();
        Token {
            token_type,
            literal,
        }
    }

    pub fn lookup_ident(ident: &str) -> TokenType {
        match ident {
            "fn" => TokenType::FUNCTION,
            "let" => TokenType::LET,
            _ => TokenType::IDENT,
        }
    }
}
