use crate::ast::{
    BlockStatement, BooleanStruct, Expression, ExpressionStatement, FunctionLiteralStruct,
    IdentifierStruct, IfExpressionStruct, InfixExpressionStruct, IntegerLiteralStruct,
    LetStatement, PrefixExpressionStruct, Program, ReturnStatement, Statement,
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
    fn new(error: String) -> Self {
        Self(error)
    }
}

pub struct Parser {
    l: Lexer,
    current_token: Token,
    peek_token: Token,
    errors: Vec<ParserError>,
}

impl Parser {
    pub fn new(mut l: Lexer) -> Self {
        // Get the first two tokens for Parser
        let current_token = l.next_token();
        let peek_token = l.next_token();

        Self {
            l,
            current_token,
            peek_token,
            errors: Vec::new(),
        }
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
    pub fn parse_program(&mut self) -> Program {
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
    // TODO: Options everywhere! Probably should remove eventually
    fn parse_expression(&mut self, precedence: i32) -> Option<Expression> {
        let mut left_exp = self.prefix_parse_fns(self.current_token.token_type.clone());
        if left_exp.is_none() {
            self.no_prefix_parse_fn_error(self.current_token.token_type.clone());
            return None;
        }

        while !self.peek_token_is(TokenType::Semicolon) && precedence < self.peek_precedence() {
            let infix_fn_exists = Self::check_infix_parse_fns(self.peek_token.token_type.clone());
            if infix_fn_exists.is_none() {
                return left_exp;
            }

            self.next_token();

            left_exp =
                self.infix_parse_fns(self.current_token.token_type.clone(), left_exp.unwrap());
        }

        left_exp
    }

    // TODO: tmp Option return type until we implement all TokenTypes
    fn prefix_parse_fns(&mut self, token_type: TokenType) -> Option<Expression> {
        match token_type {
            TokenType::Ident => Some(self.parse_identifier()),
            TokenType::Int => Some(self.parse_integer_literal()),
            TokenType::Bang => Some(self.parse_prefix_expression()),
            TokenType::Minus => Some(self.parse_prefix_expression()),
            TokenType::True => Some(self.parse_boolean_expression()),
            TokenType::False => Some(self.parse_boolean_expression()),
            TokenType::LParen => self.parse_grouped_expression(),
            TokenType::If => self.parse_if_expression(),
            TokenType::Function => self.parse_function_literal(),
            _ => None,
        }
    }

    fn parse_identifier(&mut self) -> Expression {
        Expression::Identifier(IdentifierStruct::new(
            self.current_token.clone(),
            self.current_token.literal.clone(),
        ))
    }

    fn parse_integer_literal(&mut self) -> Expression {
        let value = match self.current_token.literal.parse::<i32>() {
            Ok(val) => Some(val),
            Err(_) => {
                let msg = format!("Could not parse {} as integer", self.current_token.literal);
                self.errors.push(ParserError::new(msg));
                None
            }
        };

        Expression::IntegerLiteral(IntegerLiteralStruct::new(self.current_token.clone(), value))
    }

    fn no_prefix_parse_fn_error(&mut self, t: TokenType) {
        let msg = format!("No prefix parse function found for {:?}", t);
        self.errors.push(ParserError(msg));
    }

    fn parse_prefix_expression(&mut self) -> Expression {
        let token = self.current_token.clone();
        let operator = self.current_token.literal.clone();

        self.next_token();

        let right = self.parse_expression(PREFIX).unwrap();

        Expression::PrefixExpression(PrefixExpressionStruct::new(token, operator, right))
    }

    fn parse_boolean_expression(&mut self) -> Expression {
        Expression::Boolean(BooleanStruct::new(
            self.current_token.clone(),
            matches!(self.current_token.token_type, TokenType::True),
        ))
    }

    fn parse_grouped_expression(&mut self) -> Option<Expression> {
        self.next_token();

        let expression = self.parse_expression(LOWEST);

        if !self.expect_peek(TokenType::RParen) {
            None
        } else {
            expression
        }
    }

    fn parse_if_expression(&mut self) -> Option<Expression> {
        let if_token = self.current_token.clone();

        if !self.expect_peek(TokenType::LParen) {
            return None;
        };

        self.next_token();
        let condition = self.parse_expression(LOWEST);

        if !self.expect_peek(TokenType::RParen) {
            return None;
        };

        if !self.expect_peek(TokenType::LBrace) {
            return None;
        };

        let consequence = self.parse_block_statement();

        let mut alternative = None;

        if self.peek_token_is(TokenType::Else) {
            self.next_token();

            if !self.expect_peek(TokenType::LBrace) {
                return None;
            }

            alternative = Some(self.parse_block_statement());
        }

        Some(Expression::IfExpression(IfExpressionStruct::new(
            if_token,
            condition.unwrap(), // TODO: handle this better?
            consequence,
            alternative,
        )))
    }

    fn parse_function_literal(&mut self) -> Option<Expression> {
        let fn_token = self.current_token.clone();

        if !self.expect_peek(TokenType::LParen) {
            return None;
        }

        let fn_params = match self.parse_function_parameters() {
            Some(params) => params,
            None => return None,
        };

        if !self.expect_peek(TokenType::LBrace) {
            return None;
        }

        let fn_body = self.parse_block_statement();

        Some(Expression::FunctionExpression(FunctionLiteralStruct::new(
            fn_token, fn_params, fn_body,
        )))
    }

    fn parse_function_parameters(&mut self) -> Option<Vec<IdentifierStruct>> {
        let mut identifiers = Vec::<IdentifierStruct>::new();

        if self.peek_token_is(TokenType::RParen) {
            self.next_token();
            return Some(identifiers);
        }

        self.next_token();

        let first_identifier = IdentifierStruct::new(
            self.current_token.clone(),
            self.current_token.literal.clone(),
        );

        identifiers.push(first_identifier);

        while self.peek_token_is(TokenType::Comma) {
            self.next_token();
            self.next_token();

            let ident = IdentifierStruct::new(
                self.current_token.clone(),
                self.current_token.literal.clone(),
            );

            identifiers.push(ident);
        }

        if !self.expect_peek(TokenType::RParen) {
            return None;
        }

        Some(identifiers)
    }

    fn parse_block_statement(&mut self) -> BlockStatement {
        let token = self.current_token.clone();
        let mut statements = Vec::new();

        self.next_token();

        while !self.cur_token_is(TokenType::RBrace) && !self.cur_token_is(TokenType::Eof) {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
            self.next_token();
        }

        BlockStatement::new(token, statements)
    }

    // TODO: tmp Option return type until we implement all TokenTypes
    fn infix_parse_fns(
        &mut self,
        token_type: TokenType,
        left_expression: Expression,
    ) -> Option<Expression> {
        match token_type {
            TokenType::Plus => Some(self.parse_infix_expression(left_expression)),
            TokenType::Minus => Some(self.parse_infix_expression(left_expression)),
            TokenType::Slash => Some(self.parse_infix_expression(left_expression)),
            TokenType::Asterisk => Some(self.parse_infix_expression(left_expression)),
            TokenType::Eq => Some(self.parse_infix_expression(left_expression)),
            TokenType::NotEq => Some(self.parse_infix_expression(left_expression)),
            TokenType::Lt => Some(self.parse_infix_expression(left_expression)),
            TokenType::Gt => Some(self.parse_infix_expression(left_expression)),
            _ => None,
        }
    }
    // Should match the infix_parse_fns' match arms above.
    // Needed because parse_expression needs to check if the corresponding infix_parse_fn
    // exists first, and if it does, call next_token() before calling infix_parse_fns on the
    // token that it checked.
    fn check_infix_parse_fns(token_type: TokenType) -> Option<()> {
        match token_type {
            TokenType::Plus => Some(()),
            TokenType::Minus => Some(()),
            TokenType::Slash => Some(()),
            TokenType::Asterisk => Some(()),
            TokenType::Eq => Some(()),
            TokenType::NotEq => Some(()),
            TokenType::Lt => Some(()),
            TokenType::Gt => Some(()),
            _ => None,
        }
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Expression {
        let token = self.current_token.clone();
        let operator = self.current_token.literal.clone();

        let precedence = self.cur_precedence();
        self.next_token();
        let right = self.parse_expression(precedence).unwrap();

        Expression::InfixExpression(InfixExpressionStruct::new(token, left, operator, right))
    }

    fn precedences(token_type: TokenType) -> i32 {
        match token_type {
            TokenType::Eq => EQUALS,
            TokenType::NotEq => EQUALS,
            TokenType::Lt => LESSGREATER,
            TokenType::Gt => LESSGREATER,
            TokenType::Plus => SUM,
            TokenType::Minus => SUM,
            TokenType::Slash => PRODUCT,
            TokenType::Asterisk => PRODUCT,
            _ => LOWEST,
        }
    }

    fn peek_precedence(&self) -> i32 {
        Self::precedences(self.peek_token.token_type.clone())
    }

    fn cur_precedence(&self) -> i32 {
        Self::precedences(self.current_token.token_type.clone())
    }
}

pub fn check_parser_errors(p: Parser) {
    let errors = p.errors();
    if errors.is_empty() {
        return;
    }
    println!("Parser has {} errors.", errors.len());
    errors.iter().for_each(|err| {
        println!("Parser error: {}", err.0);
    });

    // TODO: should this panic?
    panic!()
}

#[cfg(test)]
mod tests {
    use std::any::{type_name_of_val, Any};

    use crate::ast::{Expression, Node, Statement};
    use crate::lexer::Lexer;
    use crate::parser::{check_parser_errors, Parser};

    struct ExpectedIdentifier(String);

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
        let l = Lexer::new(input);
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
        false
    }

    #[test]
    fn test_return_statements() {
        let input = r#"
return 5;
return 10;
return 993322;
"#;

        let l = Lexer::new(input);
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

    fn extract_expression(statements: Vec<Statement>) -> Expression {
        let stmt = statements.first().expect("Did not have any statements.");

        let expression_stmt = match stmt {
            Statement::Expression(s) => s,
            s => panic!(
                "program.statements[0] is not an ExpressionStatement, got {:?}",
                s
            ),
        };

        expression_stmt.expression.as_ref().unwrap().clone()
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";

        let l = Lexer::new(input);
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

        let ident_expression = extract_expression(program.statements);
        let ident = match ident_expression {
            Expression::Identifier(ref i) => i,
            e => panic!("expression not Identifier, got {:?}", e),
        };

        assert_eq!(
            ident.value, "foobar",
            "ident.value not 'foobar', got {}",
            ident.value
        );
        assert_eq!(
            ident_expression.token_literal(),
            "foobar",
            "ident token_literal() not 'foobar', got {}",
            ident_expression.token_literal()
        );
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = "5;";

        let l = Lexer::new(input);
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

        let integer_literal_expression = extract_expression(program.statements);
        let integer_literal = match integer_literal_expression {
            Expression::IntegerLiteral(ref i) => i,
            e => panic!("expression not IntegerLiteral, got {:?}", e),
        };

        assert_eq!(
            integer_literal.value.unwrap(),
            5,
            "literal.value not 5, got {}",
            integer_literal.value.unwrap()
        );
        assert_eq!(
            integer_literal_expression.token_literal(),
            "5",
            "literal.token_literal() not 5, got {}",
            integer_literal_expression.token_literal()
        );
    }

    struct PrefixTest<'a> {
        input: String,
        operator: String,
        value: &'a dyn Any,
    }
    impl PrefixTest<'_> {
        fn new<'a>(input: &'a str, operator: &'a str, value: &'a dyn Any) -> PrefixTest<'a> {
            PrefixTest {
                input: input.to_string(),
                operator: operator.to_string(),
                value,
            }
        }
    }
    #[test]
    fn test_parsing_prefix_expressions() {
        let prefix_tests: Vec<PrefixTest> = vec![
            PrefixTest::new("!5;", "!", &5),
            PrefixTest::new("-15;", "-", &15),
            PrefixTest::new("!true;", "!", &true),
            PrefixTest::new("!false;", "!", &false),
        ];

        prefix_tests.into_iter().for_each(|test| {
            let l = Lexer::new(&test.input);
            let mut p = Parser::new(l);
            let program = p.parse_program();
            check_parser_errors(p);

            assert!(
                program.statements.len() == 1,
                "program.statements does not contain 1 statement. Got: {}. Statements: {:?}",
                program.statements.len(),
                program.statements
            );

            let prefix_expression = extract_expression(program.statements);
            let prefix = match prefix_expression {
                Expression::PrefixExpression(p) => p,
                e => panic!("expression not PrefixExpression, got {:?}", e),
            };

            assert_eq!(
                prefix.operator, test.operator,
                "prefix_expression.operator is not {}. Got {}",
                prefix.operator, test.operator
            );

            assert!(test_literal_expression(*prefix.right, test.value));
        });
    }

    fn test_integer_literal(il_expression: Expression, value: i32) -> bool {
        if let Expression::IntegerLiteral(ref int_literal) = il_expression {
            if int_literal.value.unwrap() != value {
                println!(
                    "int_literal.value not {}, got: {}",
                    value,
                    int_literal.value.unwrap()
                );
                false
            } else if il_expression.token_literal() != value.to_string() {
                println!(
                    "il_expression.token_literal not {}, got: {}",
                    value,
                    il_expression.token_literal()
                );
                false
            } else {
                true
            }
        } else {
            println!(
                "int_literal not Expression::IntegerLiteral, got: {:?}",
                il_expression
            );
            false
        }
    }

    // TODO: understand these lifetime annotations
    struct InfixTest<'a> {
        input: String,
        left_value: &'a dyn Any,
        operator: String,
        right_value: &'a dyn Any,
    }
    impl InfixTest<'_> {
        fn new<'a>(
            input: &'a str,
            left_value: &'a dyn Any,
            operator: &'a str,
            right_value: &'a dyn Any,
        ) -> InfixTest<'a> {
            InfixTest {
                input: input.to_string(),
                operator: operator.to_string(),
                left_value,
                right_value,
            }
        }
    }
    #[test]
    fn test_parsing_infix_expressions() {
        let infix_tests: Vec<InfixTest> = vec![
            InfixTest::new("5 + 5;", &5, "+", &5),
            InfixTest::new("5 - 5;", &5, "-", &5),
            InfixTest::new("5 * 5;", &5, "*", &5),
            InfixTest::new("5 / 5;", &5, "/", &5),
            InfixTest::new("5 > 5;", &5, ">", &5),
            InfixTest::new("5 < 5;", &5, "<", &5),
            InfixTest::new("5 == 5;", &5, "==", &5),
            InfixTest::new("5 != 5;", &5, "!=", &5),
            InfixTest::new("true == true", &true, "==", &true),
            InfixTest::new("true != false", &true, "!=", &false),
            InfixTest::new("false == false", &false, "==", &false),
        ];

        infix_tests.into_iter().for_each(|test| {
            let l = Lexer::new(&test.input);
            let mut p = Parser::new(l);
            let program = p.parse_program();
            check_parser_errors(p);

            assert_eq!(
                program.statements.len(),
                1,
                "program.statements does not contain 1 statement. Got: {:?}. Statements: {:?}",
                program.statements.len(),
                program.statements
            );

            let infix_expression = extract_expression(program.statements);
            let infix = match infix_expression {
                Expression::InfixExpression(ie) => ie,
                e => panic!("expression not InfixExpression, got {:?}", e),
            };

            assert!(test_literal_expression(*infix.left, test.left_value,));
            assert_eq!(
                infix.operator, test.operator,
                "infix.operator is not {}. Got: {}",
                infix.operator, test.operator
            );
            assert!(test_literal_expression(*infix.right, test.right_value,));
        })
    }

    struct OperatorPrecedenceParsingTest {
        input: String,
        expected: String,
    }
    impl OperatorPrecedenceParsingTest {
        fn new(input: &str, expected: &str) -> Self {
            Self {
                input: input.to_string(),
                expected: expected.to_string(),
            }
        }
    }
    #[test]
    fn test_operator_precedence_parsing() {
        let tests: Vec<OperatorPrecedenceParsingTest> = vec![
            OperatorPrecedenceParsingTest::new("-a * b", "((-a) * b)"),
            OperatorPrecedenceParsingTest::new("!-a", "(!(-a))"),
            OperatorPrecedenceParsingTest::new("a + b + c", "((a + b) + c)"),
            OperatorPrecedenceParsingTest::new("a + b - c", "((a + b) - c)"),
            OperatorPrecedenceParsingTest::new("a * b * c", "((a * b) * c)"),
            OperatorPrecedenceParsingTest::new("a * b / c", "((a * b) / c)"),
            OperatorPrecedenceParsingTest::new("a + b / c", "(a + (b / c))"),
            OperatorPrecedenceParsingTest::new(
                "a + b * c + d / e - f",
                "(((a + (b * c)) + (d / e)) - f)",
            ),
            OperatorPrecedenceParsingTest::new("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
            OperatorPrecedenceParsingTest::new("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
            OperatorPrecedenceParsingTest::new("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
            OperatorPrecedenceParsingTest::new(
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ),
            OperatorPrecedenceParsingTest::new(
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ),
            OperatorPrecedenceParsingTest::new("true", "true"),
            OperatorPrecedenceParsingTest::new("false", "false"),
            OperatorPrecedenceParsingTest::new("3 > 5 == false", "((3 > 5) == false)"),
            OperatorPrecedenceParsingTest::new("3 < 5 == true", "((3 < 5) == true)"),
            OperatorPrecedenceParsingTest::new("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
            OperatorPrecedenceParsingTest::new("(5 + 5) * 2", "((5 + 5) * 2)"),
            OperatorPrecedenceParsingTest::new("2 / (5 + 5)", "(2 / (5 + 5))"),
            OperatorPrecedenceParsingTest::new("-(5 + 5)", "(-(5 + 5))"),
            OperatorPrecedenceParsingTest::new("!(true == true)", "(!(true == true))"),
        ];

        let mut num_fail = 0;

        tests.into_iter().for_each(|test| {
            let l = Lexer::new(&test.input);
            let mut p = Parser::new(l);
            let program = p.parse_program();
            check_parser_errors(p);

            let actual = program.string();

            if actual != test.expected {
                println!("Expected {:?}, got: {:?}", test.expected, actual);
                num_fail += 1;
            }
        });

        assert_eq!(num_fail, 0);
    }

    fn test_identifier(ident_expression: Expression, value: &str) -> bool {
        let identifier_literal = if let Expression::Identifier(ref matched_ident) = ident_expression
        {
            matched_ident
        } else {
            println!(
                "identifier_literal not Expression::Identifier, got: {:?}",
                ident_expression
            );
            return false;
        };

        if identifier_literal.value != value {
            println!(
                "identifier_literal.value not {}, got: {}",
                value, identifier_literal.value
            );
            return false;
        } else if ident_expression.token_literal() != value {
            println!(
                "ident_expression.token_literal not {}, got: {}",
                value,
                ident_expression.token_literal()
            );
            return false;
        }

        true
    }

    fn test_boolean_literal(bool_expr: Expression, value: bool) -> bool {
        let bool_literal = if let Expression::Boolean(ref matched_bool) = bool_expr {
            matched_bool
        } else {
            println!("bool_expr not Expression::Boolean, got: {:?}", bool_expr);
            return false;
        };

        if bool_literal.value != value {
            println!(
                "bool_literal.value not {}, got: {}",
                value, bool_literal.value
            );
            false
        } else if bool_expr.token_literal() != value.to_string() {
            println!(
                "bool_expr.token_literal not {}, got: {}",
                value,
                bool_expr.token_literal()
            );
            false
        } else {
            true
        }
    }

    fn test_literal_expression(expr: Expression, expected: &dyn Any) -> bool {
        if let Some(num) = expected.downcast_ref::<i32>() {
            test_integer_literal(expr, *num)
        } else if let Some(str) = expected.downcast_ref::<String>() {
            test_identifier(expr, str)
        } else if let Some(bool) = expected.downcast_ref::<bool>() {
            test_boolean_literal(expr, *bool)
        } else {
            println!("Type of expression not handled: {:?}", expected);
            false
        }
    }

    fn test_infix_expression(
        expr: Expression,
        left: &dyn Any,
        operator: &str,
        right: &dyn Any,
    ) -> bool {
        let op_expr = if let Expression::InfixExpression(matched_expr) = expr {
            matched_expr
        } else {
            println!(
                "Expression is not an Operator Expression. Got type: {} (value: {:?})",
                type_name_of_val(&expr),
                expr.string()
            );
            return false;
        };

        if !test_literal_expression(*op_expr.left, left) {
            return false;
        }

        if op_expr.operator != operator {
            println!(
                "op_expr.operator is not {}, got {}",
                operator, op_expr.operator
            );
            return false;
        }

        if !test_literal_expression(*op_expr.right, right) {
            return false;
        }

        true
    }

    struct BooleanExpressionTest {
        input: String,
        expected_boolean: bool,
    }

    #[test]
    fn test_boolean_expression() {
        let tests = vec![
            BooleanExpressionTest {
                input: "true;".to_string(),
                expected_boolean: true,
            },
            BooleanExpressionTest {
                input: "false;".to_string(),
                expected_boolean: false,
            },
        ];

        tests.into_iter().for_each(|test| {
            let l = Lexer::new(&test.input);
            let mut p = Parser::new(l);
            let program = p.parse_program();
            check_parser_errors(p);

            assert_eq!(
                program.statements.len(),
                1,
                "Program has not enough statements. Got: {}",
                program.statements.len()
            );

            let bool_expr = extract_expression(program.statements);
            let bool = match bool_expr {
                Expression::Boolean(ref b) => b,
                e => panic!("expression not Boolean, got {:?}", e),
            };

            assert_eq!(
                test.expected_boolean, bool.value,
                "bool.value not {}, got: {}",
                test.expected_boolean, bool.value
            );
        });
    }

    #[test]
    fn test_if_expression() {
        let input = "if (x < y) { x }";

        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_errors(p);

        assert_eq!(
            program.statements.len(),
            1,
            "Program has wrong number of statements. Got: {}",
            program.statements.len()
        );

        let stmt = extract_expression(program.statements);
        let if_exp = match stmt {
            Expression::IfExpression(ie) => ie,
            e => panic!("expression not IfExpression, got {:?}", e),
        };

        assert!(
            test_infix_expression(
                *if_exp.condition.clone(),
                &"x".to_string(),
                "<",
                &"y".to_string()
            ),
            "test_infix_expression failed. expression: {:?}",
            *if_exp.condition
        );

        assert_eq!(
            if_exp.consequence.statements.len(),
            1,
            "consequence is not 1 statement. Got: {}",
            if_exp.consequence.statements.len()
        );

        let consequence = extract_expression(if_exp.consequence.statements);

        assert!(test_identifier(consequence, "x"), "test_identifier failed.");

        if let Some(alt) = if_exp.alternative {
            panic!("if_exp.alternative was not None. Got: {:?}", alt);
        }
    }

    #[test]
    fn test_if_else_expression() {
        let input = "if (x < y) { x } else { y }";

        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_errors(p);

        assert_eq!(
            program.statements.len(),
            1,
            "Program has wrong number of statements. Got: {}",
            program.statements.len()
        );

        let stmt = extract_expression(program.statements);
        let if_exp = match stmt {
            Expression::IfExpression(ie) => ie,
            e => panic!("expression not IfExpression, got {:?}", e),
        };

        assert!(
            test_infix_expression(*if_exp.condition, &"x".to_string(), "<", &"y".to_string()),
            "test_infix_expression failed."
        );

        assert_eq!(
            if_exp.consequence.statements.len(),
            1,
            "consequence is not 1 statement. Got: {}",
            if_exp.consequence.statements.len()
        );

        let consequence = extract_expression(if_exp.consequence.statements);

        assert!(
            test_identifier(consequence, "x"),
            "test_identifier failed, no `x` found"
        );

        let alternative_statements = if_exp
            .alternative
            .expect("No alternative statements")
            .statements;

        assert_eq!(
            alternative_statements.len(),
            1,
            "alternative is not 1 statement. Got: {}",
            alternative_statements.len()
        );

        let alternative = extract_expression(alternative_statements);

        assert!(
            test_identifier(alternative, "y"),
            "test_identifier failed, no `y` found"
        )
    }

    #[test]
    fn test_function_literal_parsing() {
        let input = "fn(x, y) { x + y; }";

        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_errors(p);

        assert_eq!(
            program.statements.len(),
            1,
            "Program has wrong number of statements. Got: {}",
            program.statements.len()
        );

        let stmt = extract_expression(program.statements);
        let fn_literal = match stmt {
            Expression::FunctionExpression(fe) => fe,
            e => panic!("expression not FunctionExpression, got {:?}", e),
        };

        assert_eq!(
            fn_literal.parameters.len(),
            2,
            "Function literal parameters wrong. Want 2, got: {}",
            fn_literal.parameters.len(),
        );

        assert!(test_literal_expression(
            Expression::Identifier(*fn_literal.parameters[0].clone()),
            &"x".to_string()
        ));
        assert!(test_literal_expression(
            Expression::Identifier(*fn_literal.parameters[1].clone()),
            &"y".to_string()
        ));

        assert_eq!(
            fn_literal.body.statements.len(),
            1,
            "Function literal body statements not 1. Got: {}",
            fn_literal.body.statements.len(),
        );

        let body_stmt = extract_expression(fn_literal.body.statements);

        assert!(test_infix_expression(
            body_stmt,
            &"x".to_string(),
            "+",
            &"y".to_string()
        ));
    }

    struct FnParamTest {
        input: String,
        expected_params: Vec<String>,
    }
    impl FnParamTest {
        pub fn new(input: &str, expected_params: Vec<&str>) -> Self {
            FnParamTest {
                input: input.to_string(),
                expected_params: expected_params.iter().map(|v| v.to_string()).collect(),
            }
        }
    }

    #[test]
    fn test_function_parameter_parsing() {
        let tests: Vec<FnParamTest> = vec![
            FnParamTest::new("fn() {};", vec![]),
            FnParamTest::new("fn(x) {};", vec!["x"]),
            FnParamTest::new("fn(x, y, z) {};", vec!["x", "y", "z"]),
        ];

        for t in tests {
            let l = Lexer::new(&t.input);
            let mut p = Parser::new(l);
            let program = p.parse_program();
            check_parser_errors(p);

            let stmt = extract_expression(program.statements);
            let fn_literal = match stmt {
                Expression::FunctionExpression(fe) => fe,
                e => panic!("expression not FunctionExpression, got {:?}", e),
            };

            assert_eq!(
                fn_literal.parameters.len(),
                t.expected_params.len(),
                "Length of parameters wrong. Want {}, got {}",
                t.expected_params.len(),
                fn_literal.parameters.len()
            );

            t.expected_params.iter().enumerate().for_each(|(i, ident)| {
                test_literal_expression(
                    Expression::Identifier(*fn_literal.parameters[i].clone()),
                    ident,
                );
            });
        }
    }

    #[test]
    fn test_call_expression_parsing() {}

    #[test]
    fn test_call_expression_parameter_parsing() {}
}
