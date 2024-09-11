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
                if let Some(val) = &ls.value {
                    out.push_str(&val.string());
                }
                out.push(';');
            }
            Statement::Return(rs) => {
                let mut out = String::new();
                out.push_str(&self.token_literal());
                out.push(' ');

                // TODO: to be taken out when we can fully build expressions
                if let Some(val) = &rs.value {
                    out.push_str(&val.string());
                }
                out.push(';');
            }
            Statement::Expression(es) => {
                // TODO: to be taken out when we can fully build expressions
                if let Some(expression) = &es.expression {
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
    pub name: Expression,          // Should only ever be Expression::Identifier
    pub value: Option<Expression>, // TODO: temp Option until we parse expressions in Let
}
impl LetStatement {
    pub fn new(token: Token, name: IdentifierStruct, value: Option<Expression>) -> LetStatement {
        LetStatement {
            token,
            name: Expression::Identifier(name),
            value,
        }
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
    pub expression: Option<Expression>, // TODO: temp Option until we parse expressions in Return
}
impl ExpressionStatement {
    pub fn new(token: Token, expression: Option<Expression>) -> ExpressionStatement {
        ExpressionStatement { token, expression }
    }
}

/**************
* Expressions *
**************/
#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(IdentifierStruct),
    IntegerLiteral(IntegerLiteralStruct),
    PrefixExpression(PrefixExpressionStruct),
}
impl Expression {
    pub fn get_expression(&self) -> Option<IdentifierStruct> {
        match self {
            Expression::Identifier(i) => Some(i.clone()),
            _ => None,
        }
    }
}
impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Identifier(i) => i.token.literal.clone(),
            Expression::IntegerLiteral(i) => i.token.literal.clone(),
            Expression::PrefixExpression(pe) => pe.token.literal.clone(),
        }
    }
    fn string(&self) -> String {
        match self {
            Expression::Identifier(i) => i.value.clone(),
            Expression::IntegerLiteral(i) => i
                .value
                .expect("IntegerLiteralStruct has None value.")
                .to_string(),
            Expression::PrefixExpression(pe) => {
                let mut str_val = String::new();
                str_val.push('(');
                str_val.push_str(&pe.operator);
                str_val.push_str(&pe.right.string());
                str_val.push(')');

                str_val
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct IdentifierStruct {
    token: Token,
    pub value: String,
}
impl IdentifierStruct {
    pub fn new(token: Token, value: String) -> IdentifierStruct {
        IdentifierStruct { token, value }
    }
}

#[derive(Debug, Clone)]
pub struct IntegerLiteralStruct {
    token: Token,
    pub value: Option<i64>,
}
impl IntegerLiteralStruct {
    pub fn new(token: Token, value: Option<i64>) -> IntegerLiteralStruct {
        IntegerLiteralStruct { token, value }
    }
}

#[derive(Debug, Clone)]
pub struct PrefixExpressionStruct {
    token: Token,
    pub operator: String,
    pub right: Box<Expression>,
}
impl PrefixExpressionStruct {
    pub fn new(token: Token, operator: String, right: Expression) -> PrefixExpressionStruct {
        PrefixExpressionStruct {
            token,
            operator,
            right: Box::new(right),
        }
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

#[cfg(test)]
mod tests {
    use super::{Expression, IdentifierStruct, LetStatement, Program, Statement};
    use crate::{
        ast::Node,
        token::{Token, TokenType},
    };

    #[test]
    fn test_string() {
        let program = Program {
            statements: vec![Statement::Let(LetStatement {
                token: Token {
                    token_type: TokenType::Let,
                    literal: "let".to_string(),
                },
                name: Expression::Identifier(IdentifierStruct {
                    token: Token {
                        token_type: TokenType::Ident,
                        literal: "myVar".to_string(),
                    },
                    value: "myVar".to_string(),
                }),
                value: Some(Expression::Identifier(IdentifierStruct {
                    token: Token {
                        token_type: TokenType::Ident,
                        literal: "anotherVar".to_string(),
                    },
                    value: "anotherVar".to_string(),
                })),
            })],
        };

        assert_eq!(
            program.string(),
            "let myVar = anotherVar;",
            "program.string() wrong. Got {}",
            program.string()
        );
    }
}
