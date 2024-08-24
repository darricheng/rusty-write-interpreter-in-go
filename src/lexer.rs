use crate::token::*;

/// Lexer struct that will convert an input string into tokens.
///
/// `position` and `read_position` will be used to index into the input string.
/// We need two pointers because some tokens are more than one char long. For
/// example, the `let` keyword is three chars long. When lexing this keyword,
/// the `position` pointer will remain at the start of the `let` keyword while
/// the read_position pointer will carry on forwards to get the full picture of
/// exactly what the token is.
///
/// Using u8 for the ch field means we only support ASCII. Supporting UTF-8 would
/// require modifications to how individual characters are read.
struct Lexer {
    input: String,
    position: usize,      // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    ch: u8,               // current char under examination (byte in Go is an alias for u8)
}

impl Lexer {
    fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0, // null byte in ascii
        };
        l.read_char();
        return l;
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len().try_into().unwrap() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
        let tok: Token = match self.ch as char {
            '=' => Token::new(TokenType::ASSIGN, self.ch),
            ';' => Token::new(TokenType::SEMICOLON, self.ch),
            '(' => Token::new(TokenType::LPAREN, self.ch),
            ')' => Token::new(TokenType::RPAREN, self.ch),
            ',' => Token::new(TokenType::COMMA, self.ch),
            '+' => Token::new(TokenType::PLUS, self.ch),
            '{' => Token::new(TokenType::LBRACE, self.ch),
            '}' => Token::new(TokenType::RBRACE, self.ch),
            '\0' => Token::new(TokenType::EOF, 0),
            _ => Token::new(TokenType::ILLEGAL, self.ch),
        };

        self.read_char();

        tok
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::token::*;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";

        let tests: Vec<Token> = vec![
            Token {
                token_type: TokenType::ASSIGN,
                literal: "=".to_string(),
            },
            Token {
                token_type: TokenType::PLUS,
                literal: "+".to_string(),
            },
            Token {
                token_type: TokenType::LPAREN,
                literal: "(".to_string(),
            },
            Token {
                token_type: TokenType::RPAREN,
                literal: ")".to_string(),
            },
            Token {
                token_type: TokenType::LBRACE,
                literal: "{".to_string(),
            },
            Token {
                token_type: TokenType::RBRACE,
                literal: "}".to_string(),
            },
            Token {
                token_type: TokenType::COMMA,
                literal: ",".to_string(),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                token_type: TokenType::EOF,
                literal: "\0".to_string(),
            },
        ];

        let mut l = Lexer::new(input.to_string());

        for expected_token in tests {
            let tok: Token = l.next_token();

            assert_eq!(
                expected_token.token_type, tok.token_type,
                "token_type wrong, expected {:?}, got {:?}",
                expected_token.token_type, tok.token_type
            );
            assert_eq!(
                expected_token.literal, tok.literal,
                "literal wrong, expected {:?}, got {:?}",
                expected_token.token_type, tok.token_type
            );
        }
    }
}
