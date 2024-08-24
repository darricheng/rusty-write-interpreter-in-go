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

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok: Token = match self.ch as char {
            '=' => Token::new_from_byte(TokenType::ASSIGN, self.ch),
            ';' => Token::new_from_byte(TokenType::SEMICOLON, self.ch),
            '(' => Token::new_from_byte(TokenType::LPAREN, self.ch),
            ')' => Token::new_from_byte(TokenType::RPAREN, self.ch),
            ',' => Token::new_from_byte(TokenType::COMMA, self.ch),
            '+' => Token::new_from_byte(TokenType::PLUS, self.ch),
            '{' => Token::new_from_byte(TokenType::LBRACE, self.ch),
            '}' => Token::new_from_byte(TokenType::RBRACE, self.ch),
            '!' => Token::new_from_byte(TokenType::BANG, self.ch),
            '-' => Token::new_from_byte(TokenType::MINUS, self.ch),
            '/' => Token::new_from_byte(TokenType::SLASH, self.ch),
            '*' => Token::new_from_byte(TokenType::ASTERISK, self.ch),
            '<' => Token::new_from_byte(TokenType::LT, self.ch),
            '>' => Token::new_from_byte(TokenType::GT, self.ch),
            '\0' => Token::new_from_byte(TokenType::EOF, 0),
            _ => {
                if is_letter(self.ch) {
                    let literal = self.read_identifier();
                    return Token::new_from_str(Token::lookup_ident(literal), literal);
                } else if is_digit(self.ch) {
                    let literal = self.read_number();
                    return Token::new_from_str(TokenType::INT, literal);
                } else {
                    Token::new_from_byte(TokenType::ILLEGAL, self.ch)
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
"#;

        let tests: Vec<Token> = vec![
            Token::new_from_str(TokenType::LET, "let"),
            Token::new_from_str(TokenType::IDENT, "five"),
            Token::new_from_str(TokenType::ASSIGN, "="),
            Token::new_from_str(TokenType::INT, "5"),
            Token::new_from_str(TokenType::SEMICOLON, ";"),
            Token::new_from_str(TokenType::LET, "let"),
            Token::new_from_str(TokenType::IDENT, "ten"),
            Token::new_from_str(TokenType::ASSIGN, "="),
            Token::new_from_str(TokenType::INT, "10"),
            Token::new_from_str(TokenType::SEMICOLON, ";"),
            Token::new_from_str(TokenType::LET, "let"),
            Token::new_from_str(TokenType::IDENT, "add"),
            Token::new_from_str(TokenType::ASSIGN, "="),
            Token::new_from_str(TokenType::FUNCTION, "fn"),
            Token::new_from_str(TokenType::LPAREN, "("),
            Token::new_from_str(TokenType::IDENT, "x"),
            Token::new_from_str(TokenType::COMMA, ","),
            Token::new_from_str(TokenType::IDENT, "y"),
            Token::new_from_str(TokenType::RPAREN, ")"),
            Token::new_from_str(TokenType::LBRACE, "{"),
            Token::new_from_str(TokenType::IDENT, "x"),
            Token::new_from_str(TokenType::PLUS, "+"),
            Token::new_from_str(TokenType::IDENT, "y"),
            Token::new_from_str(TokenType::SEMICOLON, ";"),
            Token::new_from_str(TokenType::RBRACE, "}"),
            Token::new_from_str(TokenType::SEMICOLON, ";"),
            Token::new_from_str(TokenType::LET, "let"),
            Token::new_from_str(TokenType::IDENT, "result"),
            Token::new_from_str(TokenType::ASSIGN, "="),
            Token::new_from_str(TokenType::IDENT, "add"),
            Token::new_from_str(TokenType::LPAREN, "("),
            Token::new_from_str(TokenType::IDENT, "five"),
            Token::new_from_str(TokenType::COMMA, ","),
            Token::new_from_str(TokenType::IDENT, "ten"),
            Token::new_from_str(TokenType::RPAREN, ")"),
            Token::new_from_str(TokenType::SEMICOLON, ";"),
            Token::new_from_str(TokenType::BANG, "!"),
            Token::new_from_str(TokenType::MINUS, "-"),
            Token::new_from_str(TokenType::SLASH, "/"),
            Token::new_from_str(TokenType::ASTERISK, "*"),
            Token::new_from_str(TokenType::INT, "5"),
            Token::new_from_str(TokenType::SEMICOLON, ";"),
            Token::new_from_str(TokenType::INT, "5"),
            Token::new_from_str(TokenType::LT, "<"),
            Token::new_from_str(TokenType::INT, "10"),
            Token::new_from_str(TokenType::GT, ">"),
            Token::new_from_str(TokenType::INT, "5"),
            Token::new_from_str(TokenType::SEMICOLON, ";"),
            Token::new_from_str(TokenType::IF, "if"),
            Token::new_from_str(TokenType::LPAREN, "("),
            Token::new_from_str(TokenType::INT, "5"),
            Token::new_from_str(TokenType::LT, "<"),
            Token::new_from_str(TokenType::INT, "10"),
            Token::new_from_str(TokenType::RPAREN, ")"),
            Token::new_from_str(TokenType::LBRACE, "{"),
            Token::new_from_str(TokenType::RETURN, "return"),
            Token::new_from_str(TokenType::TRUE, "true"),
            Token::new_from_str(TokenType::SEMICOLON, ";"),
            Token::new_from_str(TokenType::RBRACE, "}"),
            Token::new_from_str(TokenType::ELSE, "else"),
            Token::new_from_str(TokenType::LBRACE, "{"),
            Token::new_from_str(TokenType::RETURN, "return"),
            Token::new_from_str(TokenType::FALSE, "false"),
            Token::new_from_str(TokenType::SEMICOLON, ";"),
            Token::new_from_str(TokenType::RBRACE, "}"),
            // Token::new_from_str(TokenType::INT, "10"),
            // Token::new_from_str(TokenType::EQ, "=="),
            // Token::new_from_str(TokenType::INT, "10"),
            // Token::new_from_str(TokenType::SEMICOLON, ";"),
            // Token::new_from_str(TokenType::INT, "10"),
            // Token::new_from_str(TokenType::NOT_EQ, "!="),
            // Token::new_from_str(TokenType::INT, "9"),
            // Token::new_from_str(TokenType::SEMICOLON, ";"),
            Token::new_from_str(TokenType::EOF, "\0"),
        ];

        let mut l = Lexer::new(input.to_string());

        for expected_token in tests {
            let tok: Token = l.next_token();

            println!(
                "TESTING: '{:?}' and '{:?}'",
                expected_token.token_type, tok.token_type
            );

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

            println!(
                "PASSED: '{:?}' and '{:?}'",
                expected_token.token_type, tok.token_type
            );
        }
    }
}
