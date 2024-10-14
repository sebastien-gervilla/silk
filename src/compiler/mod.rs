use bytecode::{Chunk, OperationCode};
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

pub struct Compiler<'a> {
    pub chunk: &'a mut Chunk<'a>,
}

impl<'a> Compiler<'a> {

    pub fn new(chunk: &'a mut Chunk<'a>) -> Self {
        Self {
            chunk,
        }
    }

    pub fn compile(&mut self, source: &str) -> &'a mut Chunk {
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
            ast::Expression::Infix(infix) => self.compile_infix_expression(infix),
            ast::Expression::Prefix(prefix) => self.compile_prefix_expression(prefix),
            _ => todo!()
        }
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

}