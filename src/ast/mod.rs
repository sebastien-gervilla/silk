use crate::token::Token;

pub struct Node {
    pub token: Token,
}

pub struct File {
    pub node: Node,
    pub statements: Vec<Statement>
}

// Statements
pub enum Statement {
    Let(LetStatement)
}

pub struct LetStatement {
    pub node: Node,
    pub identifier: Expression,
    pub expression: Option<Expression>
}

// Expressions
pub enum Expression {
    Identifier(Identifier),
    NumberLiteral(NumberLiteral)
}

pub struct Identifier {
    pub value: String
}

pub struct NumberLiteral {
    pub value: isize
}