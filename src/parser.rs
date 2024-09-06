use crate::ast::{
    Expression, ExpressionStatement, IdentifierStruct, LetStatement, Program, ReturnStatement,
    Statement,
};
use crate::token::TokenType;
use crate::{lexer::Lexer, token::Token};

/**
* Operator Precedence
*/
const LOWEST: i32 = 1;
const EQUALS: i32 = 2; // ==
const LESSGREATER: i32 = 3; // > or <
const SUM: i32 = 4; // +
const PRODUCT: i32 = 5; // *
const PREFIX: i32 = 6; // -X or !X
const CALL: i32 = 7; // my_function(X)

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
        let mut p = Parser {
            l,
            current_token: Token::new_placeholder(),
            peek_token: Token::new_placeholder(),
            errors: Vec::new(),
        };

        // Read two tokens so that current and peek tokens are set to valid tokens
        p.next_token();
        p.next_token();

        p
    }

    /**
     * Error handling
     */
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
    /**
     * Advance token
     */
    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }
    /**
     * Parse program
     */
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

    /**
     * Helper methods for checking tokens
     */
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

    /**
     * Methods for parsing
     */
    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token.token_type {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
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

        let statement = Statement::Let(LetStatement::new(let_token, statement_name, None));

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

        let statement = Statement::Return(ReturnStatement::new(return_token, None));

        Some(statement)
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expression_token = self.current_token.clone();
        let expression = self.parse_expression(LOWEST);

        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token()
        }

        let statement = Statement::Expression(ExpressionStatement::new(
            expression_token,
            Some(expression)?,
        ));

        Some(statement)
    }

    /**
     * Parse expressions
     */
    // TODO: tmp Option return type until we implement all TokenTypes
    fn prefix_parse_fns(&mut self, token_type: TokenType) -> Option<Expression> {
        match token_type {
            TokenType::Ident => Some(self.parse_identifier()),
            _ => None,
        }
    }

    // TODO: tmp Option return type until we implement all TokenTypes
    fn infix_parse_fns(token_type: TokenType, expression: Expression) -> Option<Expression> {
        match token_type {
            _ => None,
        }
    }

    // TODO: Options everywhere! Probably should remove eventually
    fn parse_expression(&mut self, precedence: i32) -> Option<Expression> {
        let left_exp = self.prefix_parse_fns(self.current_token.token_type.clone());

        left_exp
    }

    fn parse_identifier(&mut self) -> Expression {
        Expression::Identifier(IdentifierStruct::new(
            self.current_token.clone(),
            self.current_token.literal.clone(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{Expression, Node, Statement};
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
            "Program.statements does not contain 3 statements, got: {}. Statements: {:?}",
            program.statements.len(),
            program.statements
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
            if statement_data.name.get_expression().value != name {
                println!(
                    "let_statement.name.value not {}, got {}",
                    name,
                    statement_data.name.get_expression().value
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
            "Program.statements does not contain 3 statements, got: {}. Statements: {:?}",
            program.statements.len(),
            program.statements
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
            if let Statement::Return(_) = statement {
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

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";

        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_errors(p);

        assert_eq!(
            program.statements.len(),
            1,
            "program doesn't have 1 statement, got {}. Statements: {:?}",
            program.statements.len(),
            program.statements
        );

        let stmt = program.statements.get(0).unwrap();
        let expression_stmt = match stmt {
            Statement::Expression(s) => s,
            s => panic!(
                "program.statements[0] is not an ExpressionStatement, got {:?}",
                s
            ),
        };

        let ident_test = expression_stmt.expression.as_ref().unwrap();
        let ident = match ident_test {
            Expression::Identifier(i) => i,
            i => panic!("expression not Identifier, got {:?}", i),
        };

        assert_eq!(
            ident.value, "foobar",
            "ident.value not 'foobar', got {}",
            ident.value
        );
        assert_eq!(
            ident_test.token_literal(),
            "foobar",
            "ident token_literal() not 'foobar', got {}",
            ident_test.token_literal()
        );
    }
}
