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
    Let(LetStatement),
    Expression(ExpressionStatement),
}

pub struct LetStatement {
    pub node: Node,
    pub identifier: Box<Expression>,
    pub expression: Option<Box<Expression>>,
}

pub struct ExpressionStatement {
    pub node: Node,
    pub expression: Box<Expression>,
}

// Expressions
pub enum Expression {
    Identifier(Identifier),
    NumberLiteral(NumberLiteral),
    StringLiteral(StringLiteral),
    BooleanLiteral(BooleanLiteral),
    Prefix(PrefixExpression),
    Infix(InfixExpression),
}

pub struct Identifier {
    pub value: String
}

pub struct NumberLiteral {
    pub value: isize
}

pub struct StringLiteral {
    pub value: String
}

pub struct BooleanLiteral {
    pub value: bool
}

pub struct PrefixExpression {
    pub node: Node,
    pub operator: String,
    pub expression: Box<Expression>,
}

pub struct InfixExpression {
    pub node: Node,
    pub operator: String,
    pub left_expression: Box<Expression>,
    pub right_expression: Box<Expression>,
}