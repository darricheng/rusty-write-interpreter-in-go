use crate::ast::{IdentifierStruct, LetStatement, Program, ReturnStatement, Statement};
use crate::token::TokenType;
use crate::{lexer::Lexer, token::Token};

#[derive(Clone)]
struct ParserError(String);
impl ParserError {
    fn new(error: String) -> ParserError {
        ParserError(error)
    }
}

struct Parser {
    l: Lexer,
    current_token: Token,
    peek_token: Token,
    errors: Vec<ParserError>,
}

impl Parser {
    fn new(l: Lexer) -> Parser {
        Parser {
            l,
            current_token: Token::new_placeholder(),
            peek_token: Token::new_placeholder(),
            errors: Vec::new(),
        }
    }

    fn errors(&self) -> Vec<ParserError> {
        self.errors.clone()
    }

    fn peek_error(&mut self, t: TokenType) {
        let error_message = format!(
            "Expected next token to be {:?}, got {:?} instead.",
            { t },
            self.peek_token.token_type
        );
        self.errors.push(ParserError::new(error_message));
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    fn parse_program(&mut self) -> Program {
        let mut program = Program::new();

        while !self.cur_token_is(TokenType::Eof) {
            let statement = self.parse_statement();
            if let Some(stmt) = statement {
                program.statements.push(stmt);
            }
            self.next_token();
        }

        program
    }

    fn cur_token_is(&self, t: TokenType) -> bool {
        self.current_token.token_type == t
    }

    fn peek_token_is(&self, t: TokenType) -> bool {
        self.peek_token.token_type == t
    }

    fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.peek_token_is(t.clone()) {
            self.next_token();
            return true;
        }

        self.peek_error(t);

        false
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token.token_type {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        let let_token = self.current_token.clone();

        if !self.expect_peek(TokenType::Ident) {
            return None;
        }

        let statement_name = IdentifierStruct::new(
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

        let statement = Statement::Let(LetStatement::new(let_token.clone(), statement_name, None));

        Some(statement)
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        let return_token = self.current_token.clone();

        self.next_token();

        // TODO: Skipping the expressions until we encounter
        // a semicolon
        while !self.cur_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        let statement = Statement::Return(ReturnStatement::new(return_token.clone(), None));

        Some(statement)
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{Node, Program, Statement};
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    struct ExpectedIdentifier(String);

    fn check_parser_errors(p: Parser) {
        let errors = p.errors();
        if errors.len() == 0 {
            return;
        }
        println!("Parser has {} errors.", errors.len());
        errors.iter().for_each(|err| {
            println!("Parser error: {}", err.0);
        });
        panic!()
    }

    #[test]
    fn test_let_statements() {
        //         let input = r#"
        // let x 5;
        // let = 10;
        // let 838383;
        // "#;
        let input = r#"
let x = 5;
let y = 10;
let foobar = 838383;
"#;
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);

        let program = p.parse_program();

        check_parser_errors(p);

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
            return true;
        }

        println!("Statement is not Let, got {:?}", s);
        return false;
    }

    #[test]
    fn test_return_statements() {
        let input = r#"
return 5;
return 10;
return 993322;
"#;

        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);

        let program = p.parse_program();
        check_parser_errors(p);

        assert!(
            program.statements.len() == 3,
            "Program.statements does not contain 3 statements, got: {}",
            program.statements.len()
        );

        let mut fail_count = 0;

        program.statements.iter().for_each(|statement| {
            if statement.token_literal() != "return" {
                println!(
                    "return_statement.token_literal not 'return', got: {}",
                    statement.token_literal()
                );
                fail_count += 1;
            }
            if let Statement::Return(return_statement) = statement {
            } else {
                println!("statement is not a ReturnStatement. Got {:?}", statement);
                fail_count += 1;
            }
        });
        assert_eq!(
            fail_count, 0,
            "More than one return statement test failed, check logs above this."
        );
    }
}
