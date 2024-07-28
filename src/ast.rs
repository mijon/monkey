use crate::Token;

trait Node {
    fn token_literal(&self) -> String;
}

trait Statement: Node {}

trait Expression: Node {}

struct Program {
    statements: Vec<Box<dyn Statement>>,
}

struct LetStatement {
    token: Token,
    name: Identifier,
    value: dyn Expression,
}

struct Identifier {
    token: Token,
    value: String,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.tokentype.literal()
    }
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if &self.statements.len() > &0 {
            self.statements[0].token_literal()
        } else {
            "".to_string()
        }
    }
}
