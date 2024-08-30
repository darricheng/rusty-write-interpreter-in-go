use crate::ast::{self, Identifier, Program, Statement, StatementData};
use crate::token::TokenType;
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

    fn parse_program(&mut self) -> Result<Program, &str> {
        let mut program = Program::new();

        while !self.cur_token_is(TokenType::Eof) {
            let statement = self.parse_statement();
            if let Some(stmt) = statement {
                program.statements.push(stmt);
            }
            self.next_token();
        }

        Ok(program)
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token.token_type {
            TokenType::Let => self.parse_let_statement(),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        let let_token = self.current_token.clone();

        if !self.expect_peek(TokenType::Ident) {
            return None;
        }

        let statement_name = Identifier::new(
            self.current_token.clone(),
            self.current_token.literal.clone(),
        );

        if !self.expect_peek(TokenType::Assign) {
            return None;
        }

        // TODO: Skipping the expressions until we encounter
        // a semicolon
        while !self.cur_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        let statement = Statement::Let(StatementData::new(let_token.clone(), statement_name, None));

        Some(statement)
    }

    fn cur_token_is(&self, t: TokenType) -> bool {
        self.current_token.token_type == t
    }

    fn peek_token_is(&self, t: TokenType) -> bool {
        self.peek_token.token_type == t
    }

    fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{self, Node, Statement};
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    struct ExpectedIdentifier(String);

    #[test]
    fn test_let_statements() {
        let input = r#"
let x = 5;
let y = 10;
let foobar = 838383;
"#;
        let mut l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);

        let program = p
            .parse_program()
            .expect("parse_program() returned an error");

        assert!(
            program.statements.len() == 3,
            "Program.statements does not contain 3 statements, got: {}",
            program.statements.len()
        );

        let tests: Vec<ExpectedIdentifier> = vec![
            ExpectedIdentifier("x".to_string()),
            ExpectedIdentifier("y".to_string()),
            ExpectedIdentifier("foobar".to_string()),
        ];

        for (i, expected_identifier) in tests.iter().enumerate() {
            let statement = program
                .statements
                .get(i)
                .expect("Failed to index into program.statements");

            assert!(test_let_statement(statement, expected_identifier.0.clone()));
        }
    }
    fn test_let_statement(s: &Statement, name: String) -> bool {
        if s.token_literal() != "let" {
            println!("token_literal is not 'let', got {}", s.token_literal());
            return false;
        }

        if let Statement::Let(statement_data) = s {
            if statement_data.name.value != name {
                println!(
                    "let_statement.name.value not {}, got {}",
                    name, statement_data.name.value
                );
                return false;
            }
            if statement_data.name.token_literal() != name {
                println!(
                    "let_statement.name not {}, got {:?}",
                    name, statement_data.name
                );
                return false;
            }
            println!("PASSED: {:?}", statement_data);
            return true;
        }

        println!("Statement is not Let, got {:?}", s);
        return false;
    }
}
