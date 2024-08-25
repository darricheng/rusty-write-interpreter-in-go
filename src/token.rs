use std::str::{self, from_utf8};

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Illegal,
    Eof, // '\0'

    // Identifiers + literals
    Ident, // add, foobar, x, y, ...
    Int,   // 942109437

    // Operators
    Assign,   // =
    Plus,     // +
    Minus,    // -
    Bang,     // !
    Asterisk, // *
    Slash,    // /
    Lt,       // <
    Gt,       // >
    Eq,       // ==
    NotEq,    // !=

    // Delimiters
    Comma,     // ,
    Semicolon, // ;
    LParen,    // (
    RParen,    // )
    LBrace,    // {
    RBrace,    // }

    // Keywords
    Function, // fn
    Let,      // let
    True,     // true
    False,    // false
    If,       // if
    Else,     // else
    Return,   // return
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
            "fn" => TokenType::Function,
            "let" => TokenType::Let,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "return" => TokenType::Return,
            _ => TokenType::Ident,
        }
    }
}
