use crate::token::{Token, TokenType};

pub trait Node {
    fn token_literal(&self) -> String;
}

#[derive(Debug)]
pub struct StatementData {
    pub token: Token,
    pub name: Identifier,
    pub value: Expressions,
}

#[derive(Debug)]
pub enum Statement {
    Let(StatementData),
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::Let(s) => s.token.literal.clone(),
        }
    }
}

#[derive(Debug)]
pub struct Identifier {
    token: Token,
    pub value: String,
}
impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

#[derive(Debug)]
pub enum Expressions {
    Identifier(Identifier),
}

impl Node for Expressions {
    fn token_literal(&self) -> String {
        match self {
            Expressions::Identifier(i) => i.token.literal.clone(),
        }
    }
}

pub struct Program {
    pub statements: Vec<Statement>,
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
