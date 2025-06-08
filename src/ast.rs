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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    token: Token,
    value: Option<Expression>, // TODO: temp Option until we parse expressions in Return
}
impl ReturnStatement {
    pub fn new(token: Token, value: Option<Expression>) -> ReturnStatement {
        ReturnStatement { token, value }
    }
}

#[derive(Debug, Clone)]
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
    InfixExpression(InfixExpressionStruct),
    Boolean(BooleanStruct),
    IfExpression(IfExpressionStruct),
    FunctionExpression(FunctionLiteralStruct),
}
impl Expression {
    fn as_node(&self) -> &dyn Node {
        match self {
            Expression::Identifier(i) => i,
            Expression::IntegerLiteral(i) => i,
            Expression::PrefixExpression(pe) => pe,
            Expression::InfixExpression(ie) => ie,
            Expression::Boolean(b) => b,
            Expression::IfExpression(ie) => ie,
            Expression::FunctionExpression(fe) => fe,
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
    pub fn new(token: Token, value: String) -> IdentifierStruct {
        IdentifierStruct { token, value }
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
    pub fn new(token: Token, value: Option<i32>) -> IntegerLiteralStruct {
        IntegerLiteralStruct { token, value }
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
    pub fn new(token: Token, operator: String, right: Expression) -> PrefixExpressionStruct {
        PrefixExpressionStruct {
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
    pub fn new(
        token: Token,
        left: Expression,
        operator: String,
        right: Expression,
    ) -> InfixExpressionStruct {
        InfixExpressionStruct {
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
        BooleanStruct { token, value }
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
        IfExpressionStruct {
            token,
            condition: Box::new(condition),
            consequence: Box::new(consequence),
            alternative: alternative.map(|a| Box::new(a)),
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
    parameters: Vec<Box<IdentifierStruct>>,
    body: Box<BlockStatement>,
}
impl FunctionLiteralStruct {
    pub fn new(token: Token, parameters: Vec<IdentifierStruct>, body: BlockStatement) -> Self {
        FunctionLiteralStruct {
            token,
            parameters: parameters.into_iter().map(|p| Box::new(p)).collect(),
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
        BlockStatement { token, statements }
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
