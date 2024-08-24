use std::str::{self, from_utf8};

#[derive(Debug, PartialEq)]
pub enum TokenType {
    ILLEGAL,
    EOF, // '\0'

    // Identifiers + literals
    IDENT, // add, foobar, x, y, ...
    INT,   // 942109437

    // Operators
    ASSIGN,   // =
    PLUS,     // +
    MINUS,    // -
    BANG,     // !
    ASTERISK, // *
    SLASH,    // /
    LT,       // <
    GT,       // >
    EQ,       // ==
    NOT_EQ,   // !=

    // Delimiters
    COMMA,     // ,
    SEMICOLON, // ;
    LPAREN,    // (
    RPAREN,    // )
    LBRACE,    // {
    RBRACE,    // }

    // Keywords
    FUNCTION, // fn
    LET,      // let
    TRUE,     // true
    FALSE,    // false
    IF,       // if
    ELSE,     // else
    RETURN,   // return
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
