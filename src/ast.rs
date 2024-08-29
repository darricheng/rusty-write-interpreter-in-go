use crate::token::{Token, TokenType};

trait Node {
    fn token_literal(&self) -> String;
}

trait Statement: Node {
    fn statement_node(&self) {}
}

trait Expression: Node {
    fn expression_node(&self) {}
}

pub struct Program {
    statements: Vec<Box<dyn Statement>>,
}
impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements.get(0).unwrap().token_literal()
        } else {
            String::new()
        }
    }
}

struct LetStatement {
    token: Token,
    name: Identifier,
    value: dyn Expression,
}
impl Statement for LetStatement {
    fn statement_node(&self) {}
}
impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

struct Identifier {
    token: Token,
    value: String,
}
impl Expression for Identifier {
    fn expression_node(&self) {}
}
impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}
