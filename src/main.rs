use colored::Colorize;
use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::{Parser};

mod lexer;
mod parser;
mod interpreter;

fn main() {

    let lex = Lexer::new("zipette 3.5 + 34 + 21 * 2; vicer Ethan 20; zipette Ethan; vicer Jean 10; zipette Ethan + Jean;");

    let mut parser = Parser::new(lex);

    println!("===== Intepreter =====");

    match parser.parse() {
        Ok(program) => {
            Interpreter::new(program).interpret();;
        },
        Err(err) => {
            println!("{}", err.to_string().red());
        },
    };

}