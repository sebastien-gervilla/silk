use crate::token::Token;

pub struct Node {
    pub token: Token,
}

pub struct File {
    pub node: Node,
    pub statements: Vec<Statement>
}

pub enum Statement {
    Let(LetStatement)
}

pub struct LetStatement {
    pub node: Node,
    pub identifier: Identifier,
    pub expression: Option<Expression>
}

pub struct Expression {

}

pub struct Identifier {
    pub value: String
}