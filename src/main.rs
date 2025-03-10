use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;

mod lexer;
mod parser;
mod interpreter;

fn main() {
    let lex = Lexer::new("(2 + 3) * 2 ^ 2");

    let mut parser = Parser::new(lex);

    let interpreter = Interpreter::new(parser.parse_expression());

    println!("===== Intepreter =====");

    println!("Système exit : {}", interpreter.interpret());

}