pub mod tests;
pub mod types;

use std::collections::HashMap;

use types::{Type, TypeEnvironment};

use crate::ast;

// pub fn check_program(file: &ast::File) {

//     let mut environment = TypeEnvironment::new();

//     // First pass, registering function declarations
//     for statement in &file.statements {
//         if let ast::Statement::Expression(expression) = statement {
//             if let ast::Expression::Function(function) = expression.expression.as_ref() {
//                 environment.insert(function.identifier.value.clone(), function.annotation.clone());
//             }
//         }
//     }

//     // Second pass
//     for statement in &file.statements {
//         check_statement(&statement, &mut environment);
//     }
// }

// fn check_statement(statement: &ast::Statement, environment: &mut TypeEnvironment) {
//     match statement {
//         ast::Statement::Let(let_statement) => check_let_statement(let_statement, environment),
//         ast::Statement::Expression(expression) => check_expression(&expression.expression, environment),
//     }
// }

// fn check_let_statement(statement: &ast::LetStatement, environment: &mut TypeEnvironment) {

//     let assigned_type = match &statement.expression {
//         Some(expression) => synthesize_expression(expression, environment),
//         None => {
//             todo!("Requires typing annotation.")
//         },
//     };

//     match statement.annotation {
//         Some(annotation ) => {
//             if annotation != assigned_type {
//                 panic!("Expected {:?}, instead got {:?}", annotation, assigned_type);
//             }
//         },
//         None => {}
//     };

//     environment.insert(statement.identifier.value.clone(), assigned_type);
// }

// fn check_expression(expression: &ast::Expression, environment: &mut TypeEnvironment) {
//     match expression {
//         ast::Expression::NumberLiteral(literal) => todo!(),
//         ast::Expression::Function(function) => check_function(function, environment),
//         ast::Expression::Infix(infix) => {
//             let expression_type = synthesize_expression(expression, environment);
//             check_infix_expression(infix, expression_type)
//         },
//         ast::Expression::Call(call) => check_call_expression(call, environment),
//         ast::Expression::Return(expression) => check_return_expression(expression, environment),
//         _ => todo!()
//     }
// }

// fn assert_expression(expression: &ast::Expression, expected_type: Type, environment: &mut TypeEnvironment) {
//     match expression {
//         ast::Expression::NumberLiteral(_) => {
//             if expected_type != Type::Integer {
//                 panic!("Expected integer, instead got {:?}", expected_type);
//             }
//         },
//         ast::Expression::Infix(expression) => assert_infix_expession(expression, expected_type, environment),
//         ast::Expression::Call(expression) => assert_call_expression(expression, expected_type, environment),
//         ast::Expression::Return(expression) => assert_return_expression(expression, expected_type, environment),
//         _ => todo!()
//     }
// }

// fn assert_infix_expession(expression: &ast::InfixExpression, expected_type: Type, environment: &mut TypeEnvironment) {
//     let expression_type = synthesize_infix_expression(expression, environment);

//     match expression.operator.as_str() {
//         "+" | "-" | "/" | "*" | "==" | "!=" | ">" | "<" => {
//             if expression_type != Type::Integer {
//                 panic!("Type error: Expected type {:?}, got {:?} instead.", Type::Integer, expression_type)
//             }
//         },
//         _ => panic!("Operator not supported.")
//     }

//     if expression_type != expected_type {
//         panic!("Expected {:?}, instead got {:?}", expression_type, expected_type);
//     }
// }

// fn assert_call_expression(expression: &ast::CallExpression, expected_type: Type, environment: &mut TypeEnvironment) {
//     let identifier = match expression.identifier.as_ref() {
//         ast::Expression::Identifier(identifier) => identifier,
//         _ => panic!("Call expression identifier must be explicit."),
//     };

//     let return_type = match environment.get(&identifier.value) {
//         Some(return_type) => *return_type,
//         None => panic!("Couldn't find function {}", identifier.value)
//     };

//     if return_type != expected_type {
//         panic!("Expected {:?}, instead got {:?}", return_type, expected_type);
//     }
// }

// fn assert_return_expression(expression: &ast::ReturnExpression, expected_type: Type, environment: &mut TypeEnvironment) {
//     let return_type = synthesize_expression(&expression.expression, environment);

//     if return_type != expected_type {
//         panic!("Expected {:?}, instead got {:?}", return_type, expected_type);
//     }
// }

// fn check_function(function: &ast::Function, environment: &mut TypeEnvironment) {
//     let body = match function.body.as_ref() {
//         ast::Expression::Block(block) => block,
//         _ => todo!(),
//     };

//     let mut environment = TypeEnvironment::new();
//     for parameter in &function.parameters {
//         environment.insert(parameter.identifier.value.clone(), parameter.annotation);
//     }

//     for statement in &body.statements {
//         match statement {
//             ast::Statement::Let(let_statement) => check_let_statement(let_statement, &mut environment),
//             ast::Statement::Expression(expression) => check_expression(expression.expression.as_ref(), &mut environment),
//         }
//     }
// }

// fn check_infix_expression(expression: &ast::InfixExpression, expression_type: Type) {
//     match expression.operator.as_str() {
//         "+" | "-" | "/" | "*" | "==" | "!=" | ">" | "<" => {
//             if expression_type != Type::Integer {
//                 panic!("Type error: Expected type {:?}, got {:?} instead.", Type::Integer, expression_type)
//             }
//         },
//         _ => panic!("Operator not supported.")
//     }
// }

// fn check_call_expression(expression: &ast::CallExpression, environment: &mut TypeEnvironment) {
//     let identifier = match expression.identifier.as_ref() {
//         ast::Expression::Identifier(identifier) => identifier,
//         _ => panic!("Call expression identifier must be explicit."),
//     };

//     let function_type = match environment.get(&identifier.value) {
//         Some(function_type) => function_type,
//         None => panic!("Couldn't find function {}", &identifier.value)
//     };

//     // TODO: Check params
// }

// // Synthesizing

// fn synthesize_expression(expression: &ast::Expression, environment: &mut TypeEnvironment) -> Type {
//     match expression {
//         ast::Expression::NumberLiteral(_) => Type::Integer,
//         ast::Expression::BooleanLiteral(_) => Type::Boolean,
//         ast::Expression::Infix(infix) => synthesize_infix_expression(infix, environment),
//         ast::Expression::Call(call) => synthesize_call_expression(call, environment),
//         _ => todo!()
//     }
// }

// fn synthesize_infix_expression(expression: &ast::InfixExpression, environment: &mut TypeEnvironment) -> Type {
//     let left_type = synthesize_expression(&expression.left_expression, environment);
//     let right_type = synthesize_expression(&expression.right_expression, environment);
//     if left_type != right_type {
//         panic!("Type mismatch in infix expression: {:?} != {:?}", left_type, right_type)
//     }

//     return left_type
// }

// fn synthesize_call_expression(expression: &ast::CallExpression, environment: &mut TypeEnvironment) -> Type {
//     let identifier = match expression.identifier.as_ref() {
//         ast::Expression::Identifier(identifier) => identifier,
//         _ => panic!("Call expression identifier must be explicit."),
//     };

//     match environment.get(&identifier.value) {
//         Some(function_type) => *function_type,
//         None => panic!("Couldn't find function {}", &identifier.value)
//     }
// }


// HERE

enum SymbolKind {
    Variable,
    Function,
}

#[derive(Clone)]
enum Symbol {
    Variable(VariableSymbol),
    Function(FunctionSymbol),
}

#[derive(Clone)]
struct VariableSymbol {
    name: String,
    variable_type: Type,
}

#[derive(Clone)]
struct FunctionSymbol {
    name: String,
    return_type: Type,
    parameters: Vec<Type>,
}

type Scope = HashMap<String, Symbol>;

struct SymbolTable {
    scopes: Vec<Scope>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            scopes: Vec::<Scope>::new(),
        }
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn insert(&mut self, symbol: Symbol) {
        let current_scope = match self.scopes.last_mut() {
            Some(scope) => scope,
            None => panic!("No scope found.")
        };

        let symbol_name = get_symbol_name(&symbol);
        current_scope.insert(symbol_name, symbol);
    }

    pub fn get(&self, name: &str) -> Option<&Symbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(symbol) = scope.get(name) {
                return Some(symbol)
            }
        }

        return None
    }

}

fn get_symbol_name(symbol: &Symbol) -> String {
    match symbol {
        Symbol::Function(function) => function.name.clone(),
        Symbol::Variable(variable) => variable.name.clone(),
    }
}

pub fn check_program(file: &ast::File) {
    check_file(file);
}

pub fn check_file(file: &ast::File) {
    let mut symbol_table = SymbolTable::new();

    // Global scope
    symbol_table.enter_scope();

    // First pass, which involves top-level declarations
    for statement in &file.statements {
        if let ast::Statement::Expression(expression) = statement {
            if let ast::Expression::Function(function) = expression.expression.as_ref() {
                let mut parameters_types = Vec::<Type>::new();
                for parameter in &function.parameters {
                    parameters_types.push(parameter.annotation.clone());
                }

                symbol_table.insert(
                    Symbol::Function(
                        FunctionSymbol {
                            name: function.identifier.value.clone(),
                            return_type: function.annotation.clone(),
                            parameters: parameters_types,
                        }
                    )
                );
            }
        }
    }

    // Second pass
    for statement in &file.statements {
        check_statement(&mut symbol_table, statement);
    }
}

fn check_statement(symbol_table: &mut SymbolTable, statement: &ast::Statement) {
    match statement {
        ast::Statement::Let(let_statement) => check_let_statement(symbol_table, let_statement),
        ast::Statement::Expression(expression) => {
            let expected_type = synthesize_expression(symbol_table, &expression.expression);
            check_expression(symbol_table, &expression.expression, expected_type)
        },
    }
}

fn check_let_statement(symbol_table: &mut SymbolTable, statement: &ast::LetStatement) {

    let assigned_type = match &statement.expression {
        Some(expression) => synthesize_expression(symbol_table, expression),
        None => {
            todo!("Requires typing annotation.")
        },
    };

    match &statement.annotation {
        Some(annotation ) => {
            if annotation != &assigned_type {
                panic!("Expected {:?}, instead got {:?}", annotation, assigned_type);
            }
        },
        None => {}
    };

    symbol_table.insert(
        Symbol::Variable(
            VariableSymbol {
                name: statement.identifier.value.clone(),
                variable_type: assigned_type
            }
        )
    );
}

fn check_expression(symbol_table: &mut SymbolTable, expression: &ast::Expression, expected_type: Type) {
    match expression {
        ast::Expression::NumberLiteral(_) => {
            if expected_type != Type::Integer {
                panic!("Expected {:?}, instead got {:?}", expected_type, Type::Integer);
            }
        },
        ast::Expression::Function(function) => check_function(symbol_table, function),
        ast::Expression::Infix(expression) => check_infix_expession(symbol_table, expression, expected_type),
        ast::Expression::Block(expression) => check_block_expression(symbol_table, expression, expected_type),
        ast::Expression::Call(expression) => check_call_expression(symbol_table, expression, expected_type),
        // ast::Expression::Return(expression) => assert_return_expression(expression, expected_type, environment),
        _ => todo!()
    }
}

fn check_function(symbol_table: &mut SymbolTable, function: &ast::Function) {

    let body = match function.body.as_ref() {
        ast::Expression::Block(block) => block,
        _ => todo!("Function body must be a BlockExpression."), // Later we may have lambdas
    };

    symbol_table.enter_scope();

    for parameter in &function.parameters {
        symbol_table.insert(
            Symbol::Variable(
                VariableSymbol {
                    name: parameter.identifier.value.clone(),
                    variable_type: parameter.annotation.clone(),
                }
            )
        );
    }

    for statement in &body.statements {
        check_statement(symbol_table, statement);
    }

    symbol_table.exit_scope();
}

fn check_infix_expession(symbol_table: &SymbolTable, expression: &ast::InfixExpression, expected_type: Type) {
    let left_type = synthesize_expression(symbol_table, &expression.left_expression);
    let right_type = synthesize_expression(symbol_table, &expression.right_expression);

    if left_type != right_type {
        panic!("Type mismatch in infix expression: {:?} != {:?}", left_type, right_type);
    }

    match expression.operator.as_str() {
        "+" | "-" | "/" | "*" => {
            if left_type != Type::Integer {
                panic!("Type error: Expected type {:?}, got {:?} instead.", Type::Integer, left_type)
            }
        },
        "==" | "!=" | ">" | "<" => {
            if left_type != Type::Boolean {
                panic!("Type error: Expected type {:?}, got {:?} instead.", Type::Integer, left_type)
            }
        },
        _ => panic!("Operator not supported.")
    }

    if expected_type != left_type {
        panic!("Expected {:?}, instead got {:?}", expected_type, left_type);
    }
}

fn check_block_expression(symbol_table: &mut SymbolTable, expression: &ast::BlockExpression, expected_type: Type) {

    for statement in &expression.statements {
        check_statement(symbol_table, statement);
    };

    match &expression.statements[0] {
        ast::Statement::Let(_) => {
            if expected_type != Type::Void {
                panic!("Expected {:?}, instead got {:?}", expected_type, Type::Void);
            }
        },
        ast::Statement::Expression(expression) => {
            let body_type = synthesize_expression(symbol_table, &expression.expression);
            if expected_type != body_type {
                panic!("Expected {:?}, instead got {:?}", expected_type, body_type);
            }
        }
    }
}

fn check_call_expression(symbol_table: &mut SymbolTable, expression: &ast::CallExpression, expected_type: Type) {

    let get_symbol_result = match expression.identifier.as_ref() {
        ast::Expression::Identifier(identifier) => symbol_table.get(&identifier.value).cloned(),
        _ => todo!(),
    };

    let function_symbol = match get_symbol_result {
        Some(symbol) => {
            match symbol {
                Symbol::Function(function) => function,
                _ => panic!("Expected function"),
            }
        },
        None => panic!("Function not found"),
    };

    if function_symbol.return_type != expected_type {
        panic!("Expected {:?}, instead got {:?}", expected_type, function_symbol.return_type);
    }

    if expression.arguments.len() != function_symbol.parameters.len() {
        panic!("Expected {:?} arguments, instead got {:?}", function_symbol.parameters.len(), expression.arguments.len());
    }

    for index in 0..expression.arguments.len() {
        check_expression(
            symbol_table, 
            expression.arguments[index].as_ref(), 
            function_symbol.parameters[index].clone() // TODO: reference instead of clone
        );
    }
}

// Synthesizing

fn synthesize_expression(symbol_table: &SymbolTable, expression: &ast::Expression) -> Type {
    match expression {
        ast::Expression::NumberLiteral(_) => Type::Integer,
        ast::Expression::BooleanLiteral(_) => Type::Boolean,
        ast::Expression::Function(_) => Type::Void,
        // ast::Expression::Block(expression) => synthesize_block_expression(symbol_table, expression),
        ast::Expression::Infix(expression) => synthesize_infix_expression(expression),
        ast::Expression::Call(expression) => synthesize_call_expression(symbol_table, expression),
        _ => todo!(),
    }
}

fn synthesize_infix_expression(expression: &ast::InfixExpression) -> Type {
    match expression.operator.as_str() {
        "+" | "-" | "*" | "/" => Type::Integer,
        "==" | "!=" | ">" | "<" => Type::Boolean,
        operator => panic!("Invalid operator {:?} found", operator),
    }
}

fn synthesize_call_expression(symbol_table: &SymbolTable, expression: &ast::CallExpression) -> Type {
    let get_symbol_result = match expression.identifier.as_ref() {
        ast::Expression::Identifier(identifier) => symbol_table.get(&identifier.value),
        _ => todo!(),
    };

    let function_symbol = match get_symbol_result {
        Some(symbol) => {
            match symbol {
                Symbol::Function(function) => function,
                _ => panic!("Expected function"),
            }
        },
        None => panic!("Function not found"),
    };

    return function_symbol.return_type.clone()
}
