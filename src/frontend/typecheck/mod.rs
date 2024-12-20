pub mod tests;
pub mod types;

use std::collections::HashMap;

use types::Type;

use super::ast;

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

type Symbols = HashMap<String, Symbol>;

struct Scope {
    symbols: Symbols,
    return_type: Type,
}

struct SymbolTable {
    scopes: Vec<Scope>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            scopes: Vec::<Scope>::new(),
        }
    }

    pub fn get_current_scope(&self) -> &Scope {
        return self.scopes.last().expect("No scope found.")
    }

    pub fn enter_scope(&mut self) {
        let current_scope_type = match self.scopes.last() {
            Some(scope) => scope.return_type.clone(),
            None => Type::Void, // This is the global scope return type
        };

        self.scopes.push(Scope {
            symbols: Symbols::new(),
            return_type: current_scope_type,
        });
    }

    pub fn enter_function_scope(&mut self, return_type: Type) {
        self.scopes.push(Scope {
            symbols: Symbols::new(),
            return_type,
        });
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
        current_scope.symbols.insert(symbol_name, symbol);
    }

    pub fn get(&self, name: &str) -> Option<&Symbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(symbol) = scope.symbols.get(name) {
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

fn declare_scope_functions(symbol_table: &mut SymbolTable, statements: &Vec<ast::Statement>) {
    for statement in statements {
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
}

pub fn check_program(file: &ast::File) {
    check_file(file);
}

pub fn check_file(file: &ast::File) {
    let mut symbol_table = SymbolTable::new();

    // Global scope
    symbol_table.enter_scope();

    // We first declare functions so their can be used before their declaration
    declare_scope_functions(&mut symbol_table, &file.statements);
    
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
        Some(expression) => {
            let assigned_type = synthesize_expression(symbol_table, expression);
            check_expression(symbol_table, expression, assigned_type.clone());
            assigned_type
        },
        None => {
            todo!("Requires typing annotation.")
        },
    };

    match &statement.annotation {
        Some(annotation ) => {
            if annotation != &assigned_type && assigned_type != Type::None {
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
        ast::Expression::Identifier(identifier) => check_identifier(symbol_table, identifier, expected_type),
        ast::Expression::NumberLiteral(_) => {
            if expected_type != Type::Integer {
                panic!("Expected {:?}, instead got {:?}", expected_type, Type::Integer);
            }
        },
        ast::Expression::BooleanLiteral(_) => {
            if expected_type != Type::Boolean {
                panic!("Expected {:?}, instead got {:?}", expected_type, Type::Boolean);
            }
        },
        ast::Expression::StringLiteral(_) => {
            if expected_type != Type::String {
                panic!("Expected {:?}, instead got {:?}", expected_type, Type::String);
            }
        },
        ast::Expression::Function(function) => check_function(symbol_table, function),
        ast::Expression::Prefix(expression) => check_prefix_expession(symbol_table, expression, expected_type),
        ast::Expression::Infix(expression) => check_infix_expession(symbol_table, expression, expected_type),
        ast::Expression::Assign(expression) => check_assignment_expression(symbol_table, expression, expected_type),
        ast::Expression::Array(expression) => check_array_expression(symbol_table, expression, expected_type),
        ast::Expression::Block(expression) => check_block_expression(symbol_table, expression, expected_type),
        ast::Expression::If(expression) => check_if_expression(symbol_table, expression, expected_type),
        ast::Expression::While(expression) => check_while_expression(symbol_table, expression, expected_type),
        ast::Expression::Break(expression) => check_break_expression(symbol_table, expression, expected_type),
        ast::Expression::Call(expression) => check_call_expression(symbol_table, expression, expected_type),
        ast::Expression::Return(expression) => check_return_expression(symbol_table, expression),
        ast::Expression::Index(expression) => check_index_expression(symbol_table, expression, expected_type),
        _ => todo!()
    }
}

fn check_identifier(symbol_table: &mut SymbolTable, identifier: &ast::Identifier, expected_type: Type) {
    let variable_type = synthesize_identifier(symbol_table, identifier);

    if expected_type != variable_type {
        panic!("Expected {:?}, instead got {:?}", expected_type, variable_type);
    }
}

fn check_function(symbol_table: &mut SymbolTable, function: &ast::Function) {

    let body = match function.body.as_ref() {
        ast::Expression::Block(block) => block,
        _ => todo!("Function body must be a BlockExpression."),
    };

    symbol_table.enter_function_scope(function.annotation.clone());

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

    // We first declare functions so their can be used before their declaration
    declare_scope_functions(symbol_table, &body.statements);

    for statement in &body.statements {
        check_statement(symbol_table, statement);
    }

    if function.annotation != Type::Void {
        match &body.statements[body.statements.len() - 1] {
            ast::Statement::Let(_) => {
                panic!("Expected {:?}, instead got {:?}", function.annotation, Type::Void);
            },
            ast::Statement::Expression(expression) => {
                check_expression(symbol_table, &expression.expression, function.annotation.clone());
            }
        }
    }

    symbol_table.exit_scope();
}

fn check_prefix_expession(symbol_table: &mut SymbolTable, expression: &ast::PrefixExpression, expected_type: Type) {
    match expression.operator.as_str() {
        "!" => {
            check_expression(symbol_table, &expression.expression, Type::Boolean);

            if expected_type != Type::Boolean {
                panic!("Type error: Expected type {:?}, got {:?} instead.", expected_type, Type::Boolean)
            }
        },
        "-" => {
            check_expression(symbol_table, &expression.expression, Type::Integer);

            if expected_type != Type::Integer {
                panic!("Type error: Expected type {:?}, got {:?} instead.", expected_type, Type::Integer)
            }
        },
        operator => panic!("Invalid operator {:?} found", operator),
    }
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

            if expected_type != Type::Integer {
                panic!("Expected {:?}, instead got {:?}", expected_type, Type::Integer);
            }
        },
        ">" | "<" => {
            if left_type != Type::Integer {
                panic!("Type error: Expected type {:?}, got {:?} instead.", Type::Integer, left_type)
            }
            
            if expected_type != Type::Boolean {
                panic!("Expected {:?}, instead got {:?}", expected_type, Type::Boolean);
            }
        },
        "==" | "!=" | "&&" | "||" => {
            if expected_type != Type::Boolean {
                panic!("Expected {:?}, instead got {:?}", expected_type, Type::Boolean);
            }
        },
        _ => panic!("Operator not supported.")
    }
}

fn check_assignment_expression(symbol_table: &mut SymbolTable, expression: &ast::AssignmentExpression, expected_type: Type) {
    if expected_type != Type::Void {
        panic!("Type error: Expected type {:?}, got {:?} instead.", expected_type, Type::Void)
    }
    
    let variable_type = synthesize_identifier(symbol_table, &expression.identifier);
    check_expression(symbol_table, &expression.expression, variable_type);
}

fn check_array_expression(symbol_table: &mut SymbolTable, expression: &ast::ArrayExpression, expected_type: Type) {
    let array_type = match expected_type {
        Type::Array(array_type) => array_type,
        _ => panic!("Type error: Expected type {:?}, got array instead.", expected_type),
    };

    for element in &expression.elements {
        // TODO: "Type" should be passed as reference
        check_expression(symbol_table, element, *array_type.clone());
    }
}

fn check_block_expression(symbol_table: &mut SymbolTable, expression: &ast::BlockExpression, expected_type: Type) {

    for statement in &expression.statements {
        check_statement(symbol_table, statement);
    };

    match &expression.statements[expression.statements.len() - 1] {
        ast::Statement::Let(_) => {
            if expected_type != Type::Void {
                panic!("Expected {:?}, instead got {:?}", expected_type, Type::Void);
            }
        },
        ast::Statement::Expression(expression) => {
            check_expression(symbol_table, &expression.expression, expected_type);
        }
    }
}

fn check_if_expression(symbol_table: &mut SymbolTable, expression: &ast::IfExpression, expected_type: Type) {
    check_expression(symbol_table, &expression.condition, Type::Boolean);

    symbol_table.enter_scope();
    check_expression(symbol_table, &expression.consequence, expected_type.clone());
    symbol_table.exit_scope();
    
    if let Some(alternative) = &expression.alternative {
        symbol_table.enter_scope();
        check_expression(symbol_table, &alternative, expected_type);
        symbol_table.exit_scope();
    };
}

fn check_while_expression(symbol_table: &mut SymbolTable, expression: &ast::WhileExpression, _: Type) {
    check_expression(symbol_table, &expression.condition, Type::Boolean);

    match &expression.iteration.as_ref() {
        ast::Expression::Block(expression) => {
            symbol_table.enter_scope();
            for statement in &expression.statements {
                check_statement(symbol_table, statement);
            };
            symbol_table.exit_scope();
        },
        _ => panic!("Expected BlockExpression."), // TODO: This should be put in semantic analysis
    }
}

fn check_break_expression(_: &mut SymbolTable, _: &ast::BreakExpression, _: Type) {
    // TODO: This needs semantic analysis, to check if we're in a loop
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

fn check_return_expression(symbol_table: &SymbolTable, expression: &ast::ReturnExpression) {
    let current_scope = symbol_table.get_current_scope();

    let return_type = synthesize_expression(symbol_table, &expression.expression);
    if current_scope.return_type != return_type {
        panic!("Type error: Expected type {:?}, got {:?} instead.", current_scope.return_type, return_type);
    }
}

fn check_index_expression(symbol_table: &mut SymbolTable, expression: &ast::IndexExpression, expected_type: Type) {
    check_expression(symbol_table, &expression.index, Type::Integer);
    let indexed_type = synthesize_expression(symbol_table, &expression.indexed);

    match indexed_type {
        Type::Array(array_type) => {
            if array_type.as_ref() != &expected_type {
                panic!("Type error: Expected type {:?}, got {:?} instead.", expected_type, array_type);
            }
        },
        actual_type => panic!("Type error: Expected type array, got {:?} instead.", actual_type),
    }
}

// Synthesizing

fn synthesize_expression(symbol_table: &SymbolTable, expression: &ast::Expression) -> Type {
    match expression {
        ast::Expression::Identifier(identifier) => synthesize_identifier(symbol_table, identifier),
        ast::Expression::NumberLiteral(_) => Type::Integer,
        ast::Expression::BooleanLiteral(_) => Type::Boolean,
        ast::Expression::StringLiteral(_) => Type::String,
        ast::Expression::Function(_) => Type::Void,
        ast::Expression::Prefix(expression) => synthesize_prefix_expression(expression),
        ast::Expression::Infix(expression) => synthesize_infix_expression(expression),
        ast::Expression::Assign(expression) => synthesize_assignment_expression(expression),
        ast::Expression::Array(expression) => synthesize_array_expression(symbol_table, expression),
        ast::Expression::Block(expression) => synthesize_block_expression(symbol_table, expression),
        ast::Expression::If(expression) => synthesize_if_expression(symbol_table, expression),
        ast::Expression::While(_) => Type::Void,
        ast::Expression::Break(_) => {
            return Type::None; // Just as return, it doesn't hold the value
        },
        ast::Expression::Call(expression) => synthesize_call_expression(symbol_table, expression),
        ast::Expression::Return(_) => {
            return Type::None;
            // TODO: synthesize_expression must return an optional type
            // synthesize_expression(symbol_table, &expression.expression)
        },
        ast::Expression::Index(expression) => synthesize_index_expression(symbol_table, expression),
        _ => todo!(),
    }
}

fn synthesize_identifier(symbol_table: &SymbolTable, identifier: &ast::Identifier) -> Type {
    let variable_symbol = match symbol_table.get(&identifier.value) {
        Some(symbol) => {
            match symbol {
                Symbol::Variable(variable) => variable,
                _ => panic!("Expected variable"),
            }
        },
        None => panic!("Variable not found"),
    };

    return variable_symbol.variable_type.clone()
}

fn synthesize_prefix_expression(expression: &ast::PrefixExpression) -> Type {
    match expression.operator.as_str() {
        "-" => Type::Integer,
        "!" => Type::Boolean,
        operator => panic!("Invalid operator {:?} found", operator),
    }
}

fn synthesize_infix_expression(expression: &ast::InfixExpression) -> Type {
    match expression.operator.as_str() {
        "+" | "-" | "*" | "/" => Type::Integer,
        "==" | "!=" | ">" | "<" => Type::Boolean,
        "&&" | "||" => Type::Boolean,
        operator => panic!("Invalid operator {:?} found", operator),
    }
}

fn synthesize_assignment_expression(_: &ast::AssignmentExpression) -> Type {
    return Type::Void // TODO: Assignment expressions may return the assigned value
}

// The type of the array should be determined on the first element
fn synthesize_array_expression(symbol_table: &SymbolTable, expression: &ast::ArrayExpression) -> Type {
    let array_type = synthesize_expression(symbol_table, &expression.elements[0]);
    return Type::Array(Box::new(array_type))
}

fn synthesize_block_expression(symbol_table: &SymbolTable, expression: &ast::BlockExpression) -> Type {
    match &expression.statements[expression.statements.len() - 1] {
        ast::Statement::Let(_) => Type::Void,
        ast::Statement::Expression(expression) => {
            synthesize_expression(symbol_table, &expression.expression)
        }
    }
}

fn synthesize_if_expression(symbol_table: &SymbolTable, expression: &ast::IfExpression) -> Type {
    let consequence_type = synthesize_expression(symbol_table, &expression.consequence);
    
    match &expression.alternative {
        Some(alternative) => {
            let alternative_type = synthesize_expression(symbol_table, alternative.as_ref());

            if consequence_type == Type::None {
                return alternative_type;
            }

            if alternative_type == Type::None {
                return consequence_type;
            }

            if consequence_type != alternative_type {
                panic!("Type mismatch in if expression: {:?} != {:?}", consequence_type, alternative_type);
            }

            consequence_type
        },
        None => consequence_type,
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

fn synthesize_index_expression(symbol_table: &SymbolTable, expression: &ast::IndexExpression) -> Type {
    let indexed_type = synthesize_expression(symbol_table, &expression.indexed);

    match indexed_type {
        Type::Array(array_type) => *array_type,
        actual_type => panic!("Type error: Expected type array, got {:?} instead.", actual_type),
    }
}
