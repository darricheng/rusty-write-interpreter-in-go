use crate::token::{self, Token, TokenType};

pub trait Node {
    fn token_literal(&self) -> String;
}

/*************
* Statements *
*************/
#[derive(Debug)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::Let(s) => s.token.literal.clone(),
            Statement::Return(s) => s.token.literal.clone(),
            Statement::Expression(s) => s.token.literal.clone(),
        }
    }
}

#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub name: IdentifierStruct,
    pub value: Option<Expression>, // TODO: temp Option until we parse expressions in Let
}
impl LetStatement {
    pub fn new(token: Token, name: IdentifierStruct, value: Option<Expression>) -> LetStatement {
        LetStatement { token, name, value }
    }
}

#[derive(Debug)]
pub struct ReturnStatement {
    token: Token,
    value: Option<Expression>, // TODO: temp Option until we parse expressions in Return
}
impl ReturnStatement {
    pub fn new(token: Token, value: Option<Expression>) -> ReturnStatement {
        ReturnStatement { token, value }
    }
}

#[derive(Debug)]
pub struct ExpressionStatement {
    token: Token,
    expression: Expression,
}

#[derive(Debug)]
pub struct IdentifierStruct {
    token: Token,
    pub value: String,
}
impl IdentifierStruct {
    pub fn new(token: Token, value: String) -> IdentifierStruct {
        IdentifierStruct { token, value }
    }
}
impl Node for IdentifierStruct {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

#[derive(Debug)]
pub enum Expression {
    Identifier(IdentifierStruct),
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
