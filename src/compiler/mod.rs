use bytecode::{Chunk, OperationCode};

use crate::{
    ast, lexer::Lexer, parser::{
        parse_file, 
        Parser
    }
};

pub mod tests;
pub mod bytecode;
pub mod debug;
pub mod value;
pub mod vm;

pub struct Compiler<'a> {
    pub chunk: &'a mut Chunk,
}

impl<'a> Compiler<'a> {

    pub fn new(chunk: &'a mut Chunk) -> Self {
        Self {
            chunk,
        }
    }

    pub fn compile(&mut self, source: &str) {
        let mut lexer = Lexer::new(source);
        let mut parser = Parser::new(&mut lexer);

        let ast = parse_file(&mut parser);

        self.compile_file(&ast);
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
                self.chunk.add_constant(literal.value as f64, 1);
            },
            ast::Expression::Infix(infix) => self.compile_infix_expression(infix),
            ast::Expression::Prefix(prefix) => self.compile_prefix_expression(prefix),
            _ => todo!()
        }
    }

    fn compile_prefix_expression(&mut self, expression: &ast::PrefixExpression) {
        self.compile_expression(&expression.expression);

        match expression.operator.as_str() {
            "-" => self.chunk.add_operation(OperationCode::NEGATE, 1),
            _ => todo!()
        }
    }

    fn compile_infix_expression(&mut self, expression: &ast::InfixExpression) {

        self.compile_expression(&expression.left_expression);
        self.compile_expression(&expression.right_expression);

        match expression.operator.as_str() {
            "+" => self.chunk.add_operation(OperationCode::ADD, 1),
            "-" => self.chunk.add_operation(OperationCode::SUBSTRACT, 1),
            "*" => self.chunk.add_operation(OperationCode::MULTIPLY, 1),
            "/" => self.chunk.add_operation(OperationCode::DIVIDE, 1),
            _ => todo!(),
        }
    }

}