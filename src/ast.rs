trait Node {
    fn token_literal(&self) -> String;
}

trait Statement: Node {
    fn statement_node(&self) {}
}

trait Expression: Node {
    fn expression_node(&self) {}
}

struct Program {
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
