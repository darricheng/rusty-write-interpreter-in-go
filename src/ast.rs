use crate::token::Token;

pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

/*************
* Statements *
*************/
#[derive(Debug, Clone)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Self::Let(s) => s.token.literal.clone(),
            Self::Return(s) => s.token.literal.clone(),
            Self::Expression(s) => s.token.literal.clone(),
        }
    }
    fn string(&self) -> String {
        let mut out = String::new();
        match self {
            Self::Let(ls) => {
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
            Self::Return(rs) => {
                let mut out = String::new();
                out.push_str(&self.token_literal());
                out.push(' ');

                // TODO: to be taken out when we can fully build expressions
                if let Some(val) = &rs.value {
                    out.push_str(&val.string());
                }
                out.push(';');
            }
            Self::Expression(es) => {
                // TODO: to be taken out when we can fully build expressions
                if let Some(expression) = &es.expression {
                    out.push_str(&expression.string());
                }
            }
        }

        out
    }
}

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub token: Token,
    pub name: IdentifierStruct,
    pub value: Option<Expression>, // TODO: temp Option until we parse expressions in Let
}
impl LetStatement {
    pub fn new(token: Token, name: IdentifierStruct, value: Option<Expression>) -> Self {
        Self { token, name, value }
    }
}

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    token: Token,
    value: Option<Expression>, // TODO: temp Option until we parse expressions in Return
}
impl ReturnStatement {
    pub fn new(token: Token, value: Option<Expression>) -> Self {
        Self { token, value }
    }
}

#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    token: Token,
    pub expression: Option<Expression>, // TODO: temp Option until we parse expressions in Return
}
impl ExpressionStatement {
    pub fn new(token: Token, expression: Option<Expression>) -> Self {
        Self { token, expression }
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
    InfixExpression(InfixExpressionStruct),
    Boolean(BooleanStruct),
    IfExpression(IfExpressionStruct),
    FunctionExpression(FunctionLiteralStruct),
    CallExpression(CallExpressionStruct),
}
impl Expression {
    fn as_node(&self) -> &dyn Node {
        match self {
            Self::Identifier(i) => i,
            Self::IntegerLiteral(i) => i,
            Self::PrefixExpression(pe) => pe,
            Self::InfixExpression(ie) => ie,
            Self::Boolean(b) => b,
            Self::IfExpression(ie) => ie,
            Self::FunctionExpression(fe) => fe,
            Self::CallExpression(ce) => ce,
        }
    }
}
impl Node for Expression {
    fn token_literal(&self) -> String {
        self.as_node().token_literal()
    }
    fn string(&self) -> String {
        self.as_node().string()
    }
}

#[derive(Debug, Clone)]
pub struct IdentifierStruct {
    token: Token,
    pub value: String,
}
impl IdentifierStruct {
    pub fn new(token: Token, value: String) -> Self {
        Self { token, value }
    }
}
impl Node for IdentifierStruct {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        self.value.clone()
    }
}

#[derive(Debug, Clone)]
pub struct IntegerLiteralStruct {
    token: Token,
    pub value: Option<i32>,
}
impl IntegerLiteralStruct {
    pub fn new(token: Token, value: Option<i32>) -> Self {
        Self { token, value }
    }
}
impl Node for IntegerLiteralStruct {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        self.value
            .expect("IntegerLiteralStruct has None value.")
            .to_string()
    }
}

#[derive(Debug, Clone)]
pub struct PrefixExpressionStruct {
    token: Token,
    pub operator: String,
    pub right: Box<Expression>,
}
impl PrefixExpressionStruct {
    pub fn new(token: Token, operator: String, right: Expression) -> Self {
        Self {
            token,
            operator,
            right: Box::new(right),
        }
    }
}
impl Node for PrefixExpressionStruct {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        let mut str_val = String::new();
        str_val.push('(');
        str_val.push_str(&self.operator);
        str_val.push_str(&self.right.string());
        str_val.push(')');

        str_val
    }
}

#[derive(Debug, Clone)]
pub struct InfixExpressionStruct {
    token: Token,
    pub left: Box<Expression>,
    pub operator: String,
    pub right: Box<Expression>,
}
impl InfixExpressionStruct {
    pub fn new(token: Token, left: Expression, operator: String, right: Expression) -> Self {
        Self {
            token,
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}
impl Node for InfixExpressionStruct {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        let mut str_val = String::new();
        str_val.push('(');
        str_val.push_str(&self.left.string());
        str_val.push(' ');
        str_val.push_str(&self.operator);
        str_val.push(' ');
        str_val.push_str(&self.right.string());
        str_val.push(')');

        str_val
    }
}

#[derive(Debug, Clone)]
pub struct BooleanStruct {
    token: Token,
    pub value: bool,
}
impl BooleanStruct {
    pub fn new(token: Token, value: bool) -> Self {
        Self { token, value }
    }
}
impl Node for BooleanStruct {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        self.token.literal.clone()
    }
}

#[derive(Debug, Clone)]
pub struct IfExpressionStruct {
    token: Token,
    // TODO: is Box correct here for indirection?
    // It's probably the easiest for now though
    pub condition: Box<Expression>,
    // The Go example uses pointers. We Box the BlockStatements
    // to make this easier to use without all the lifetime annotations.
    pub consequence: Box<BlockStatement>,
    pub alternative: Option<Box<BlockStatement>>,
}
impl IfExpressionStruct {
    pub fn new(
        token: Token,
        condition: Expression,
        consequence: BlockStatement,
        alternative: Option<BlockStatement>,
    ) -> Self {
        Self {
            token,
            condition: Box::new(condition),
            consequence: Box::new(consequence),
            alternative: alternative.map(Box::new),
        }
    }
}
impl Node for IfExpressionStruct {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        let mut str_val = String::new();
        str_val.push_str("if");
        str_val.push_str(&self.condition.string());
        str_val.push(' ');
        str_val.push_str(&self.consequence.string());

        if let Some(alternative) = &self.alternative {
            str_val.push_str("else ");
            str_val.push_str(&alternative.string());
        }

        str_val
    }
}

#[derive(Debug, Clone)]
pub struct FunctionLiteralStruct {
    token: Token, // The `fn` token
    pub parameters: Vec<Box<IdentifierStruct>>,
    pub body: Box<BlockStatement>,
}
impl FunctionLiteralStruct {
    pub fn new(token: Token, parameters: Vec<IdentifierStruct>, body: BlockStatement) -> Self {
        Self {
            token,
            parameters: parameters.into_iter().map(Box::new).collect(),
            body: Box::new(body),
        }
    }
}
impl Node for FunctionLiteralStruct {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        let mut str_val = String::new();

        let params_str: String = self.parameters.iter().fold(String::new(), |acc, ident| {
            let ident_str = ident.string();
            format!("{acc}, {ident_str}")
        });

        str_val.push_str(&self.token_literal());
        str_val.push('(');
        str_val.push_str(&params_str);
        str_val.push(')');
        str_val.push_str(&self.body.string());

        str_val
    }
}

#[derive(Debug, Clone)]
pub struct CallExpressionStruct {
    token: Token,                  // The `(` token
    pub function: Box<Expression>, // NOTE: is Box correct here
    pub arguments: Vec<Expression>,
}
impl CallExpressionStruct {
    fn new(token: Token, function: Expression, arguments: Vec<Expression>) -> Self {
        Self {
            token,
            function: Box::new(function),
            arguments,
        }
    }
}
impl Node for CallExpressionStruct {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        let arguments_length = self.arguments.len() - 1;

        let args_str = self
            .arguments
            .iter()
            .enumerate()
            .map(|(i, expr)| {
                format!(
                    "{}{}",
                    expr.string(),
                    if i < arguments_length { ", " } else { "" }
                )
            })
            .collect::<String>();

        let mut str_val = String::new();
        str_val.push_str(&self.function.string());
        str_val.push('(');
        str_val.push_str(&args_str);
        str_val.push(')');

        str_val
    }
}

/*********
* Blocks *
*********/
#[derive(Debug, Clone)]
pub struct BlockStatement {
    token: Token, // the { token
    pub statements: Vec<Statement>,
}

impl BlockStatement {
    pub fn new(token: Token, statements: Vec<Statement>) -> Self {
        Self { token, statements }
    }
}

impl Node for BlockStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        let mut str_val = String::new();

        self.statements.iter().for_each(|stmt| {
            str_val.push_str(&stmt.string());
        });

        str_val
    }
}

/**********
* Program *
**********/
pub struct Program {
    pub statements: Vec<Statement>,
}
impl Program {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }
}
impl Node for Program {
    fn token_literal(&self) -> String {
        if !self.statements.is_empty() {
            self.statements.first().unwrap().token_literal()
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
                name: IdentifierStruct {
                    token: Token {
                        token_type: TokenType::Ident,
                        literal: "myVar".to_string(),
                    },
                    value: "myVar".to_string(),
                },
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
