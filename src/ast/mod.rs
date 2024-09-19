use crate::token::Token;

struct Node {
    token: Token,
}

struct LetStatement<'a> {
    node: Node,
    identifier: &'a Identifier,
    expression: Option<Expression>
}

struct Expression {

}

struct Identifier {
    value: String
}