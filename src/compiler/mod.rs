use std::array;

use bytecode::{Chunk, OperationCode};
use object::{Object, StringObject};
use value::Value;

use crate::{
    ast, lexer::Lexer, parser::{
        parse_file, 
        Parser
    }, typecheck::check_program
};

pub mod tests;
pub mod bytecode;
pub mod debug;
pub mod value;
pub mod vm;
pub mod object;

const LOCALS_SIZE: usize = 256;

pub struct Compiler<'a> {
    pub chunk: &'a mut Chunk,
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

impl<'a> Compiler<'a> {

    pub fn new(chunk: &'a mut Chunk) -> Self {
        Self {
            chunk,
            locals: array::from_fn(|_| None),
            locals_count: 0,
            depth: 0,
        }
    }

    pub fn compile(&mut self, source: &str) -> &mut Chunk {
        let mut lexer = Lexer::new(source);
        let mut parser = Parser::new(&mut lexer);

        let ast = parse_file(&mut parser);
        println!("Parsing completed.");

        check_program(&ast);
        println!("Typechecking completed.");

        self.compile_file(&ast);

        return self.chunk
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
            return;
        }

        let index = self.declare_local_variable(&statement.identifier);

        match &statement.expression {
            Some(expression) => self.compile_expression(expression.as_ref()),
            None => todo!(),
        }

        self.chunk.add_operation(OperationCode::SET_LOCAL, statement.node.token.line);
        self.chunk.add_instruction(index as u8, statement.node.token.line);

        match &mut self.locals[index] {
            Some(local) => local.is_initialized = true,
            None => panic!("Local variable not found after initialization"),
        }
    }

    fn compile_expression(&mut self, expression: &ast::Expression) {
        match expression {
            ast::Expression::Identifier(identifier) => self.compile_identifier(identifier),
            ast::Expression::NumberLiteral(literal) => {
                self.chunk.add_constant(Value::F64(literal.value as f64), literal.node.token.line);
            },
            ast::Expression::BooleanLiteral(literal) => self.compile_boolean_literal(literal),
            ast::Expression::StringLiteral(literal) => self.compile_string_literal(literal),
            ast::Expression::Infix(infix) => self.compile_infix_expression(infix),
            ast::Expression::Prefix(prefix) => self.compile_prefix_expression(prefix),
            ast::Expression::Block(expression) => self.compile_block_expression(expression),
            ast::Expression::If(expression) => self.compile_if_expression(expression),
            _ => todo!()
        }
    }

    fn compile_identifier(&mut self, identifier: &ast::Identifier) {
        let variable_index = self.get_local_variable_index(&identifier.value);

        match variable_index {
            Some(index) => {
                self.chunk.add_operation(OperationCode::GET_LOCAL, identifier.node.token.line);
                self.chunk.add_instruction(index as u8, identifier.node.token.line);
            },
            None => todo!() // TODO: We could assume this is a global variable if we support it.
        }
    }

    fn compile_string_literal(&mut self, literal: &ast::StringLiteral) {
        let string_object = StringObject {
            length: literal.value.len(),
            value: literal.value.clone()
        };
        
        self.chunk.add_constant(
            Value::Object(
                Object::String(string_object)
            ), 
            literal.node.token.line
        );
    }

    fn compile_boolean_literal(&mut self, literal: &ast::BooleanLiteral) {
        self.chunk.add_operation(
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
            "!" => self.chunk.add_operation(OperationCode::NOT, expression.node.token.line),
            "-" => self.chunk.add_operation(OperationCode::NEGATE, expression.node.token.line),
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
            "+" => self.chunk.add_operation(OperationCode::ADD, expression.node.token.line),
            "-" => self.chunk.add_operation(OperationCode::SUBSTRACT, expression.node.token.line),
            "*" => self.chunk.add_operation(OperationCode::MULTIPLY, expression.node.token.line),
            "/" => self.chunk.add_operation(OperationCode::DIVIDE, expression.node.token.line),
            "==" => self.chunk.add_operation(OperationCode::EQUALS, expression.node.token.line),
            "!=" => self.chunk.add_operation(OperationCode::NOT_EQUALS, expression.node.token.line),
            ">" => self.chunk.add_operation(OperationCode::GREATER, expression.node.token.line),
            "<" => self.chunk.add_operation(OperationCode::LESS, expression.node.token.line),
            operator => todo!("Operator {} not implemented yet.", operator),
        }
    }

    fn compile_and_expression(&mut self, expression: &ast::InfixExpression) {
        self.compile_expression(&expression.left_expression);

        let end_jump = self.chunk.add_jump(
            OperationCode::JUMP_IF_FALSE, 
            expression.node.token.line,
        );

        self.chunk.add_operation(OperationCode::POP, expression.node.token.line);
        self.compile_expression(&expression.right_expression);

        self.chunk.patch_jump(end_jump);
    }

    fn compile_or_expression(&mut self, expression: &ast::InfixExpression) {
        self.compile_expression(&expression.left_expression);

        let else_jump = self.chunk.add_jump(
            OperationCode::JUMP_IF_FALSE, 
            expression.node.token.line
        );

        let end_jump = self.chunk.add_jump(
            OperationCode::JUMP, 
            expression.node.token.line
        );

        self.chunk.patch_jump(else_jump);
        self.chunk.add_operation(
            OperationCode::POP, 
            expression.node.token.line
        );

        self.compile_expression(&expression.right_expression);
        self.chunk.patch_jump(end_jump);
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

        let then_jump = self.chunk.add_jump(OperationCode::JUMP_IF_FALSE, expression.node.token.line);
        self.chunk.add_operation(OperationCode::POP, expression.node.token.line);
        self.compile_expression(&expression.consequence);

        let alternative_jump = self.chunk.add_jump(OperationCode::JUMP, expression.node.token.line);
        self.chunk.patch_jump(then_jump);
        self.chunk.add_operation(OperationCode::POP, expression.node.token.line);

        if let Some(alternative) = &expression.alternative {
            self.compile_expression(alternative);
        }

        self.chunk.patch_jump(alternative_jump);
    }

    // Utils

    fn declare_local_variable(&mut self, identifier: &ast::Identifier) -> usize {

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
            println!("{index}");
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