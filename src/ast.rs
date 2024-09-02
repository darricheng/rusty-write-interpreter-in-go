use crate::token::Token;

pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
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
    fn string(&self) -> String {
        let mut out = String::new();
        match self {
            Statement::Let(ls) => {
                out.push_str(&self.token_literal());
                out.push(' ');
                out.push_str(&ls.name.string());
                out.push_str(" = ");

                // TODO: to be taken out when we can fully build expressions
                if let Some(val) = ls.value {
                    out.push_str(&val.string());
                }
                out.push(';');
            }
            Statement::Return(rs) => {
                let mut out = String::new();
                out.push_str(&self.token_literal());
                out.push(' ');

                // TODO: to be taken out when we can fully build expressions
                if let Some(val) = rs.value {
                    out.push_str(&val.string());
                }
                out.push(';');
            }
            Statement::Expression(es) => {
                // TODO: to be taken out when we can fully build expressions
                if let Some(expression) = es.expression {
                    out.push_str(&expression.string());
                }
            }
        }

        out
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
    expression: Option<Expression>, // TODO: temp Option until we parse expressions in Return
}

/**************
* Expressions *
**************/
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
    fn string(&self) -> String {
        match self {
            Expression::Identifier(i) => i.value,
        }
    }
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

/**********
* Program *
**********/
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
    fn string(&self) -> String {
        let mut out = String::new();

        self.statements.iter().for_each(|s| {
            out.push_str(&s.string());
        });

        out
    }
}
