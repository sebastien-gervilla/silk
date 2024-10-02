pub mod tests;
pub mod types;

use types::Type;

use crate::ast;

pub fn check_program(file: &ast::File) {
    for statement in &file.statements {
        check_statemenet(&statement);
    }
}

fn check_statemenet(statement: &ast::Statement) {
    match statement {
        ast::Statement::Expression(expression) => check_expression(&expression.expression),
        _ => todo!(),
    }
}

fn check_expression(expression: &ast::Expression) {
    let expression_type = synthesize_expression(expression);

    match expression {
        ast::Expression::NumberLiteral(literal) => todo!(),
        ast::Expression::Infix(infix) => check_infix_expression(infix, expression_type),
        _ => todo!()
    }
}

fn check_infix_expression(expression: &ast::InfixExpression, expression_type: Type) {
    match expression.operator.as_str() {
        "+" | "-" | "/" | "*" | "==" | "!=" | ">" | "<" => {
            if expression_type != Type::Integer {
                panic!("Type error: Expected type {:?}, got {:?} instead.", Type::Integer, expression_type)
            }
        },
        _ => panic!("Operator not supported.")
    }
}

// Synthesizing

fn synthesize_expression(expression: &ast::Expression) -> Type {
    match expression {
        ast::Expression::NumberLiteral(_) => Type::Integer,
        ast::Expression::BooleanLiteral(_) => Type::Boolean,
        ast::Expression::Infix(infix) => synthesize_infix_expression(infix),
        _ => todo!()
    }
}

fn synthesize_infix_expression(expression: &ast::InfixExpression) -> Type {
    let left_type = synthesize_expression(&expression.left_expression);
    let right_type = synthesize_expression(&expression.right_expression);
    if left_type != right_type {
        panic!("Type mismatch in infix expression: {:?} != {:?}", left_type, right_type)
    }

    return left_type
}