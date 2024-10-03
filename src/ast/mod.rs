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
    pub identifier: Identifier,
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
    Function(Function),
    Prefix(PrefixExpression),
    Infix(InfixExpression),
    Block(BlockExpression),
    If(IfExpression),
    While(WhileExpression),
    Call(CallExpression),
}

pub struct Identifier {
    pub node: Node,
    pub value: String
}

pub struct NumberLiteral {
    pub node: Node,
    pub value: isize,
}

pub struct StringLiteral {
    pub node: Node,
    pub value: String
}

pub struct BooleanLiteral {
    pub node: Node,
    pub value: bool
}

pub struct Function {
    pub node: Node,
    pub identifier: Identifier,
    pub parameters: Vec<FunctionParameter>,
    pub body: Box<Expression>,
}

pub struct FunctionParameter {
    pub identifier: Identifier,
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

pub struct BlockExpression {
    pub node: Node,
    pub statements: Vec<Statement>,
}

pub struct IfExpression {
    pub node: Node,
    pub condition: Box<Expression>,
    pub consequence: Box<Expression>,
    pub alternative: Option<Box<Expression>>,
}

pub struct WhileExpression {
    pub node: Node,
    pub condition: Box<Expression>,
    pub iteration: Box<Expression>,
}

pub struct CallExpression {
    pub node: Node,
    pub identifier: Box<Expression>,
    pub arguments: Vec<Box<Expression>>,
}