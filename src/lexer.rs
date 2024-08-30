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
pub struct Lexer {
    input: String,
    position: usize,      // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    ch: u8,               // current char under examination (byte in Go is an alias for u8)
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
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
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input.as_bytes()[self.read_position]
        }
    }

    fn read_identifier(&mut self) -> &str {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }

        &self.input[position..self.position]
    }

    fn read_number(&mut self) -> &str {
        let position = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }

        &self.input[position..self.position]
    }

    fn skip_whitespace(&mut self) {
        while self.ch as char == ' '
            || self.ch as char == '\t'
            || self.ch as char == '\n'
            || self.ch as char == '\r'
        {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok: Token = match self.ch as char {
            '=' => {
                // check for '=='
                if self.peek_char() == 61 {
                    let ch = self.ch as char;
                    self.read_char();
                    Token::new_from_str(TokenType::Eq, &format!("{}{}", ch, self.ch as char))
                } else {
                    Token::new_from_byte(TokenType::Assign, self.ch)
                }
            }
            '!' => {
                // check for '!='
                if self.peek_char() == 61 {
                    let ch = self.ch as char;
                    self.read_char();
                    Token::new_from_str(TokenType::NotEq, &format!("{}{}", ch, self.ch as char))
                } else {
                    Token::new_from_byte(TokenType::Bang, self.ch)
                }
            }
            ';' => Token::new_from_byte(TokenType::Semicolon, self.ch),
            '(' => Token::new_from_byte(TokenType::LParen, self.ch),
            ')' => Token::new_from_byte(TokenType::RParen, self.ch),
            ',' => Token::new_from_byte(TokenType::Comma, self.ch),
            '+' => Token::new_from_byte(TokenType::Plus, self.ch),
            '{' => Token::new_from_byte(TokenType::LBrace, self.ch),
            '}' => Token::new_from_byte(TokenType::RBrace, self.ch),
            '-' => Token::new_from_byte(TokenType::Minus, self.ch),
            '/' => Token::new_from_byte(TokenType::Slash, self.ch),
            '*' => Token::new_from_byte(TokenType::Asterisk, self.ch),
            '<' => Token::new_from_byte(TokenType::Lt, self.ch),
            '>' => Token::new_from_byte(TokenType::Gt, self.ch),
            '\0' => Token::new_from_byte(TokenType::Eof, 0),
            _ => {
                if is_letter(self.ch) {
                    let literal = self.read_identifier();
                    return Token::new_from_str(Token::lookup_ident(literal), literal);
                } else if is_digit(self.ch) {
                    let literal = self.read_number();
                    return Token::new_from_str(TokenType::Int, literal);
                } else {
                    Token::new_from_byte(TokenType::Illegal, self.ch)
                }
            }
        };

        self.read_char();

        tok
    }
}

fn is_letter(ch: u8) -> bool {
    97 <= ch && ch <= 122 || // lowercase a-z
    65 <= ch && ch <= 90 || // uppercase A-Z
    ch == 95 // underscore
}

fn is_digit(ch: u8) -> bool {
    48 <= ch && ch <= 57 // 0 to 9
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::token::*;

    #[test]
    fn test_next_token() {
        let input = r#"let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;
"#;

        let tests: Vec<Token> = vec![
            Token::new_from_str(TokenType::Let, "let"),
            Token::new_from_str(TokenType::Ident, "five"),
            Token::new_from_str(TokenType::Assign, "="),
            Token::new_from_str(TokenType::Int, "5"),
            Token::new_from_str(TokenType::Semicolon, ";"),
            Token::new_from_str(TokenType::Let, "let"),
            Token::new_from_str(TokenType::Ident, "ten"),
            Token::new_from_str(TokenType::Assign, "="),
            Token::new_from_str(TokenType::Int, "10"),
            Token::new_from_str(TokenType::Semicolon, ";"),
            Token::new_from_str(TokenType::Let, "let"),
            Token::new_from_str(TokenType::Ident, "add"),
            Token::new_from_str(TokenType::Assign, "="),
            Token::new_from_str(TokenType::Function, "fn"),
            Token::new_from_str(TokenType::LParen, "("),
            Token::new_from_str(TokenType::Ident, "x"),
            Token::new_from_str(TokenType::Comma, ","),
            Token::new_from_str(TokenType::Ident, "y"),
            Token::new_from_str(TokenType::RParen, ")"),
            Token::new_from_str(TokenType::LBrace, "{"),
            Token::new_from_str(TokenType::Ident, "x"),
            Token::new_from_str(TokenType::Plus, "+"),
            Token::new_from_str(TokenType::Ident, "y"),
            Token::new_from_str(TokenType::Semicolon, ";"),
            Token::new_from_str(TokenType::RBrace, "}"),
            Token::new_from_str(TokenType::Semicolon, ";"),
            Token::new_from_str(TokenType::Let, "let"),
            Token::new_from_str(TokenType::Ident, "result"),
            Token::new_from_str(TokenType::Assign, "="),
            Token::new_from_str(TokenType::Ident, "add"),
            Token::new_from_str(TokenType::LParen, "("),
            Token::new_from_str(TokenType::Ident, "five"),
            Token::new_from_str(TokenType::Comma, ","),
            Token::new_from_str(TokenType::Ident, "ten"),
            Token::new_from_str(TokenType::RParen, ")"),
            Token::new_from_str(TokenType::Semicolon, ";"),
            Token::new_from_str(TokenType::Bang, "!"),
            Token::new_from_str(TokenType::Minus, "-"),
            Token::new_from_str(TokenType::Slash, "/"),
            Token::new_from_str(TokenType::Asterisk, "*"),
            Token::new_from_str(TokenType::Int, "5"),
            Token::new_from_str(TokenType::Semicolon, ";"),
            Token::new_from_str(TokenType::Int, "5"),
            Token::new_from_str(TokenType::Lt, "<"),
            Token::new_from_str(TokenType::Int, "10"),
            Token::new_from_str(TokenType::Gt, ">"),
            Token::new_from_str(TokenType::Int, "5"),
            Token::new_from_str(TokenType::Semicolon, ";"),
            Token::new_from_str(TokenType::If, "if"),
            Token::new_from_str(TokenType::LParen, "("),
            Token::new_from_str(TokenType::Int, "5"),
            Token::new_from_str(TokenType::Lt, "<"),
            Token::new_from_str(TokenType::Int, "10"),
            Token::new_from_str(TokenType::RParen, ")"),
            Token::new_from_str(TokenType::LBrace, "{"),
            Token::new_from_str(TokenType::Return, "return"),
            Token::new_from_str(TokenType::True, "true"),
            Token::new_from_str(TokenType::Semicolon, ";"),
            Token::new_from_str(TokenType::RBrace, "}"),
            Token::new_from_str(TokenType::Else, "else"),
            Token::new_from_str(TokenType::LBrace, "{"),
            Token::new_from_str(TokenType::Return, "return"),
            Token::new_from_str(TokenType::False, "false"),
            Token::new_from_str(TokenType::Semicolon, ";"),
            Token::new_from_str(TokenType::RBrace, "}"),
            Token::new_from_str(TokenType::Int, "10"),
            Token::new_from_str(TokenType::Eq, "=="),
            Token::new_from_str(TokenType::Int, "10"),
            Token::new_from_str(TokenType::Semicolon, ";"),
            Token::new_from_str(TokenType::Int, "10"),
            Token::new_from_str(TokenType::NotEq, "!="),
            Token::new_from_str(TokenType::Int, "9"),
            Token::new_from_str(TokenType::Semicolon, ";"),
            Token::new_from_str(TokenType::Eof, "\0"),
        ];

        let mut l = Lexer::new(input.to_string());

        for expected_token in tests {
            let tok: Token = l.next_token();

            // println!(
            //     "TESTING: '{:?}' and '{:?}'",
            //     expected_token.token_type, tok.token_type
            // );

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

            // println!(
            //     "PASSED: '{:?}' and '{:?}'",
            //     expected_token.token_type, tok.token_type
            // );
        }
    }
}
