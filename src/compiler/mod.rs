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
            ast::Statement::Expression(expression_statement) => {
                self.compile_expression(&expression_statement.expression);
            },
            _ => todo!()
        }
    }

    fn compile_expression(&mut self, expression: &ast::Expression) {
        match expression {
            ast::Expression::NumberLiteral(literal) => {
                self.chunk.add_constant(Value::F64(literal.value as f64), literal.node.token.line);
            },
            ast::Expression::BooleanLiteral(literal) => self.compile_boolean_literal(literal),
            ast::Expression::StringLiteral(literal) => self.compile_string_literal(literal),
            ast::Expression::Infix(infix) => self.compile_infix_expression(infix),
            ast::Expression::Prefix(prefix) => self.compile_prefix_expression(prefix),
            ast::Expression::Block(expression) => self.compile_block_expression(expression),
            _ => todo!()
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
            _ => todo!(),
        }
    }

    fn compile_block_expression(&mut self, expression: &ast::BlockExpression) {
        self.depth += 1;

        for statement in &expression.statements {
            self.compile_statement(statement);
        }

        self.depth -= 1;
    }

}