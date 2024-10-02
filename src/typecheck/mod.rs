pub mod tests;
pub mod types;

use types::{Type, TypeEnvironment};

use crate::ast;

struct TypeChecker {
    
}

pub fn check_program(file: &ast::File) {

    let mut environment = TypeEnvironment::new();

    for statement in &file.statements {
        check_statemenet(&statement, &mut environment);
    }
}

fn check_statemenet(statement: &ast::Statement, environment: &mut TypeEnvironment) {
    match statement {
        ast::Statement::Let(let_statement) => check_let_statement(let_statement, environment),
        ast::Statement::Expression(expression) => check_expression(&expression.expression),
        _ => todo!(),
    }
}

fn check_let_statement(statement: &ast::LetStatement, environment: &mut TypeEnvironment) {

    let variable_type = match &statement.expression {
        Some(expression) => synthesize_expression(expression),
        None => {
            todo!("Requires typing annotation.")
        },
    };

    // TODO: Force identifier
    let variable_name = match statement.identifier.as_ref() {
        ast::Expression::Identifier(identifier) => identifier.value.clone(),
        _ => todo!("Identifier requires naming."),
    };

    environment.insert(variable_name, variable_type);
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