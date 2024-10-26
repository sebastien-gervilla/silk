use std::array;

use bytecode::{Chunk, OperationCode};
use object::{FunctionObject, Object, StringObject};
use value::Value;

use crate::ast;

pub mod tests;
pub mod bytecode;
pub mod debug;
pub mod value;
pub mod vm;
pub mod object;

const LOCALS_SIZE: usize = 256;

pub struct Compiler<'a> {
    // Top-level code is implicitly a function
    pub function: &'a mut FunctionObject,

    pub locals: [Option<Local>; LOCALS_SIZE],
    pub locals_count: usize,
    pub depth: usize,
}

#[derive(Debug)]
pub struct Local {
    pub name: String,
    pub depth: usize,
    pub is_initialized: bool,
}

pub struct GlobalFunction {
    pub arity: usize,
    pub chunk: Chunk,
    pub name: String,
}

impl<'a> Compiler<'a> {

    pub fn new(function: &'a mut FunctionObject) -> Self {
        Self {
            function,
            locals: array::from_fn(|_| None),
            locals_count: 0,
            depth: 0,
        }
    }

    pub fn get_current_chunk(&mut self) -> &mut Chunk {
        return &mut self.function.chunk
    }

    pub fn compile(&mut self, ast: &ast::File) -> &mut FunctionObject {
        self.compile_file(&ast);
        return &mut self.function
    }

    fn compile_file(&mut self, file: &ast::File) {
        for statement in &file.statements {
            self.compile_statement(statement);
        }
    }

    fn compile_statement(&mut self, statement: &ast::Statement) {
        match statement {
            ast::Statement::Let(let_statement) => self.compile_let_statement(let_statement),
            ast::Statement::Expression(expression_statement) => {
                self.compile_expression(&expression_statement.expression);
            },
        }
    }

    fn compile_let_statement(&mut self, statement: &ast::LetStatement) {
        if self.depth <= 0 {
            panic!("Cannot declare variables at top-level.")
        }

        let index = self.declare_local_variable(&statement.identifier);

        match &statement.expression {
            Some(expression) => self.compile_expression(expression.as_ref()),
            None => todo!(),
        }

        self.function.chunk.add_operation(OperationCode::SET_LOCAL, statement.node.token.line);
        self.function.chunk.add_instruction(index as u8, statement.node.token.line);

        match &mut self.locals[index] {
            Some(local) => local.is_initialized = true,
            None => panic!("Local variable not found after initialization"),
        }
    }

    fn compile_expression(&mut self, expression: &ast::Expression) {
        match expression {
            ast::Expression::Identifier(identifier) => self.compile_identifier(identifier),
            ast::Expression::NumberLiteral(literal) => self.compile_number_literal(literal),
            ast::Expression::BooleanLiteral(literal) => self.compile_boolean_literal(literal),
            ast::Expression::StringLiteral(literal) => self.compile_string_literal(literal),
            ast::Expression::Function(function) => self.compile_function(function),
            ast::Expression::Prefix(prefix) => self.compile_prefix_expression(prefix),
            ast::Expression::Infix(infix) => self.compile_infix_expression(infix),
            ast::Expression::Assign(expression) => self.compile_assignment_expression(expression),
            ast::Expression::Block(expression) => self.compile_block_expression(expression),
            ast::Expression::If(expression) => self.compile_if_expression(expression),
            ast::Expression::While(expression) => self.compile_while_expression(expression),
            ast::Expression::Call(expression) => self.compile_call_expression(expression),
            _ => todo!()
        }
    }

    fn compile_identifier(&mut self, identifier: &ast::Identifier) {
        let variable_index = self.get_local_variable_index(&identifier.value);

        match variable_index {
            Some(index) => {
                self.function.chunk.add_operation(OperationCode::GET_LOCAL, identifier.node.token.line);
                self.function.chunk.add_instruction(index as u8, identifier.node.token.line);
            },
            None => {
                // Global variables
                let constant_index = self.function.chunk.push_constant(
                    Value::Object(
                        Object::String(
                            StringObject {
                                length: identifier.value.len(),
                                value: identifier.value.clone(),
                            }
                        )
                    )
                );

                self.function.chunk.add_operation(OperationCode::GET_GLOBAL, identifier.node.token.line);
                self.function.chunk.add_instruction(constant_index, identifier.node.token.line);
            }
        }
    }

    fn compile_number_literal(&mut self, literal: &ast::NumberLiteral) {
        self.function.chunk.add_constant(
            Value::F64(literal.value as f64), 
            literal.node.token.line
        );
    }

    fn compile_string_literal(&mut self, literal: &ast::StringLiteral) {
        let string_object = StringObject {
            length: literal.value.len(),
            value: literal.value.clone()
        };
        
        self.function.chunk.add_constant(
            Value::Object(
                Object::String(string_object)
            ), 
            literal.node.token.line
        );
    }

    fn compile_function(&mut self, function: &ast::Function) {

        let mut constant: Option<u8> = None;
        if self.depth > 0 {
            let index = self.declare_local_variable(&function.identifier);

            match &mut self.locals[index] {
                Some(local) => local.is_initialized = true,
                None => panic!("Function not found after initialization"),
            }
        } else {
            let constant_index = self.function.chunk.push_constant(
                Value::Object(
                    Object::String(
                        StringObject {
                            length: function.identifier.value.len(),
                            value: function.identifier.value.clone(),
                        }
                    )
                )
            );

            constant = Some(constant_index);
        }

        let function_object = &mut FunctionObject {
            chunk: Chunk::new(),
            arity: 0,
            name: function.identifier.value.clone(),
        };
        let mut compiler = Compiler::new(function_object);

        compiler.depth += 1;
        compiler.compile_function_parameters(function);
        compiler.depth -= 1;

        compiler.compile_expression(&function.body);

        self.function.chunk.add_constant(Value::Object(Object::Function(function_object.clone())), function.node.token.line);

        if let Some(constant_index) = constant {
            self.function.chunk.add_operation(OperationCode::SET_GLOBAL, function.node.token.line);
            self.function.chunk.add_instruction(constant_index, function.node.token.line);
        }
    }

    fn compile_function_parameters(&mut self, function: &ast::Function) {
        for parameter in &function.parameters {
            self.function.arity += 1;
            let index = self.declare_local_variable(&parameter.identifier);

            match &mut self.locals[index] {
                Some(local) => local.is_initialized = true,
                None => panic!("Function not found after initialization"),
            }
        }
    }

    fn compile_boolean_literal(&mut self, literal: &ast::BooleanLiteral) {
        self.function.chunk.add_operation(
            if literal.value {
                OperationCode::TRUE
            } else {
                OperationCode::FALSE
            }, 
            literal.node.token.line
        );
    }

    fn compile_prefix_expression(&mut self, expression: &ast::PrefixExpression) {
        self.compile_expression(&expression.expression);

        match expression.operator.as_str() {
            "!" => self.function.chunk.add_operation(OperationCode::NOT, expression.node.token.line),
            "-" => self.function.chunk.add_operation(OperationCode::NEGATE, expression.node.token.line),
            _ => todo!()
        }
    }

    fn compile_infix_expression(&mut self, expression: &ast::InfixExpression) {
        match expression.operator.as_str() {
            "&&" => self.compile_and_expression(&expression),
            "||" => self.compile_or_expression(&expression),
            _ => self.compile_simple_infix_expression(&expression),
        }
    }

    fn compile_simple_infix_expression(&mut self, expression: &ast::InfixExpression) {
        self.compile_expression(&expression.left_expression);
        self.compile_expression(&expression.right_expression);

        match expression.operator.as_str() {
            "+" => self.function.chunk.add_operation(OperationCode::ADD, expression.node.token.line),
            "-" => self.function.chunk.add_operation(OperationCode::SUBSTRACT, expression.node.token.line),
            "*" => self.function.chunk.add_operation(OperationCode::MULTIPLY, expression.node.token.line),
            "/" => self.function.chunk.add_operation(OperationCode::DIVIDE, expression.node.token.line),
            "==" => self.function.chunk.add_operation(OperationCode::EQUALS, expression.node.token.line),
            "!=" => self.function.chunk.add_operation(OperationCode::NOT_EQUALS, expression.node.token.line),
            ">" => self.function.chunk.add_operation(OperationCode::GREATER, expression.node.token.line),
            "<" => self.function.chunk.add_operation(OperationCode::LESS, expression.node.token.line),
            operator => todo!("Operator {} not implemented yet.", operator),
        }
    }

    fn compile_and_expression(&mut self, expression: &ast::InfixExpression) {
        self.compile_expression(&expression.left_expression);

        let end_jump = self.function.chunk.add_jump(
            OperationCode::JUMP_IF_FALSE, 
            expression.node.token.line,
        );

        self.function.chunk.add_operation(OperationCode::POP, expression.node.token.line);
        self.compile_expression(&expression.right_expression);

        self.function.chunk.patch_jump(end_jump);
    }

    fn compile_or_expression(&mut self, expression: &ast::InfixExpression) {
        self.compile_expression(&expression.left_expression);

        let else_jump = self.function.chunk.add_jump(
            OperationCode::JUMP_IF_FALSE, 
            expression.node.token.line
        );

        let end_jump = self.function.chunk.add_jump(
            OperationCode::JUMP, 
            expression.node.token.line
        );

        self.function.chunk.patch_jump(else_jump);
        self.function.chunk.add_operation(
            OperationCode::POP, 
            expression.node.token.line
        );

        self.compile_expression(&expression.right_expression);
        self.function.chunk.patch_jump(end_jump);
    }

    fn compile_assignment_expression(&mut self, expression: &ast::AssignmentExpression) {
        let variable_index = self.get_local_variable_index(&expression.identifier.value);

        self.compile_expression(&expression.expression);

        match variable_index {
            Some(index) => {
                self.function.chunk.add_operation(OperationCode::SET_LOCAL, expression.node.token.line);
                self.function.chunk.add_instruction(index as u8, expression.node.token.line);
            },
            None => todo!() // TODO: We could assume this is a global variable if we support it.
        }
    }

    fn compile_block_expression(&mut self, expression: &ast::BlockExpression) {
        self.depth += 1;

        for statement in &expression.statements {
            self.compile_statement(statement);
        }

        self.depth -= 1;
    }

    fn compile_if_expression(&mut self, expression: &ast::IfExpression) {
        self.compile_expression(&expression.condition);

        let then_jump = self.function.chunk.add_jump(OperationCode::JUMP_IF_FALSE, expression.node.token.line);
        self.function.chunk.add_operation(OperationCode::POP, expression.node.token.line);
        self.compile_expression(&expression.consequence);

        let alternative_jump = self.function.chunk.add_jump(OperationCode::JUMP, expression.node.token.line);
        self.function.chunk.patch_jump(then_jump);
        self.function.chunk.add_operation(OperationCode::POP, expression.node.token.line);

        if let Some(alternative) = &expression.alternative {
            self.compile_expression(alternative);
        }

        self.function.chunk.patch_jump(alternative_jump);
    }

    fn compile_while_expression(&mut self, expression: &ast::WhileExpression) {
        let loop_start = self.function.chunk.code.len();

        self.compile_expression(&expression.condition);

        let exit_jump = self.function.chunk.add_jump(
            OperationCode::JUMP_IF_FALSE, 
            expression.node.token.line
        );

        self.function.chunk.add_operation(
            OperationCode::POP, 
            expression.node.token.line
        );

        self.compile_expression(&expression.iteration);
        self.function.chunk.add_loop(loop_start, expression.node.token.line);

        self.function.chunk.patch_jump(exit_jump);
        self.function.chunk.add_operation(
            OperationCode::POP, 
            expression.node.token.line
        );
    }

    fn compile_call_expression(&mut self, expression: &ast::CallExpression) {
        self.compile_expression(expression.identifier.as_ref());

        for argument in &expression.arguments {
            self.compile_expression(argument);
        }

        self.function.chunk.add_operation(OperationCode::CALL, expression.node.token.line);
        self.function.chunk.add_instruction(expression.arguments.len() as u8, expression.node.token.line);
    }


    // Utils

    fn declare_local_variable(&mut self, identifier: &ast::Identifier) -> usize {

        if self.depth == 0 {
            panic!("Cannot declare local variable at global scope");
        }

        if self.locals_count >= LOCALS_SIZE {
            panic!("Exceeded locals variable count");
        }

        self.locals[self.locals_count] = Some(
            Local {
                name: identifier.value.clone(),
                depth: self.depth,
                is_initialized: false,
            }
        );

        self.locals_count += 1;

        return self.locals_count - 1
    }

    fn get_local_variable_index(&mut self, name: &str) -> Option<usize> {
        for index in (0..self.locals_count).rev() {
            let local_option = &self.locals[index];
            if let Some(local) = local_option {
                if local.name == name {
                    if !local.is_initialized {
                        panic!("Can't read local variable in its own initializer")
                    }

                    return Some(index)
                }
            }
        }

        return None
    }

}