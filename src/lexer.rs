use crate::token::*;

/// Lexer struct that will convert an input string into tokens.
///
/// `position` and `read_position` will be used to index into the input string.
/// We need two pointers because some tokens are more than one char long. For
/// example, the `let` keyword is three chars long. When lexing this keyword,
/// the `position` pointer will remain at the start of the `let` keyword while
/// the read_position pointer will carry on forwards to get the full picture of
/// exactly what the token is.
struct Lexer {
    input: String,
    position: usize,      // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    ch: u8,               // current char under examination (byte in Go is an alias for u8)
}

impl Lexer {
    fn new(input: String) -> Lexer {
        Lexer {
            input,
            // TODO: not sure if the below default initialisations work
            position: 0,
            read_position: 0,
            ch: 0,
        }
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
                token_type: ASSIGN,
                literal: "=".to_string(),
            },
            Token {
                token_type: PLUS,
                literal: "+".to_string(),
            },
            Token {
                token_type: LPAREN,
                literal: "(".to_string(),
            },
            Token {
                token_type: RPAREN,
                literal: ")".to_string(),
            },
            Token {
                token_type: LBRACE,
                literal: "{".to_string(),
            },
            Token {
                token_type: RBRACE,
                literal: "}".to_string(),
            },
            Token {
                token_type: COMMA,
                literal: ",".to_string(),
            },
            Token {
                token_type: SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                token_type: EOF,
                literal: "".to_string(),
            },
        ];

        let l = Lexer::new(input.to_string());

        for expected_token in tests {
            let tok: Token = l.next_token();

            assert_eq!(
                expected_token.token_type, tok.token_type,
                "token_type wrong, expected {}, got {}",
                expected_token.token_type, tok.token_type
            );
            assert_eq!(
                expected_token.literal, tok.literal,
                "literal wrong, expected {}, got {}",
                expected_token.token_type, tok.token_type
            );
        }
    }
}
