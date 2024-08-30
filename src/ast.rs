use crate::token::{Token, TokenType};

pub trait Node {
    fn token_literal(&self) -> String;
}

#[derive(Debug)]
pub struct StatementData {
    pub token: Token,
    pub name: Identifier,
    pub value: Option<Expression>, // TODO: temp Option until we parse expressions in Let
}
impl StatementData {
    pub fn new(token: Token, name: Identifier, value: Option<Expression>) -> StatementData {
        StatementData { token, name, value }
    }
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
impl Identifier {
    pub fn new(token: Token, value: String) -> Identifier {
        Identifier { token, value }
    }
}
impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

#[derive(Debug)]
pub enum Expression {
    Identifier(Identifier),
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Identifier(i) => i.token.literal.clone(),
        }
    }
}

pub struct Program {
    pub statements: Vec<Statement>,
}
impl Program {
    pub fn new() -> Program {
        Program {
            statements: Vec::new(),
        }
    }
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
