use std::env;
use std::fmt::format;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use colored::Colorize;
use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::{Parser};

mod lexer;
mod parser;
mod interpreter;

const DEFAULT_FILE: &str = "quartier";
const EXTENSION: &str = "zipette";
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let mut args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        args.insert(1, format!("{DEFAULT_FILE}.{EXTENSION}"));
    }

    if !&args[1].ends_with(EXTENSION) {
        println!("{}", format!("File must be a .{EXTENSION} file.").red());
        std::process::exit(1);
    }

    let filename = Path::new(&args[1]);

    if !filename.exists() {
        println!("{}", format!("File {} do not exist.", &args[1]).red());
        std::process::exit(1);
    }


    let mut file = File::open(filename).inspect_err(|err|{
        println!("{}", format!("Failed to open file {} : {}", &args[1], err).red());
        std::process::exit(1);
    }).unwrap();

    let mut file_content = String::new();

    match file.read_to_string(&mut file_content) {
        Ok(_) => {},
        Err(err) => {
            println!("{}", format!("Failed to read file : {}", err).red());
            std::process::exit(1);
        }
    }


    let lex = Lexer::new(file_content.as_str());

    &lex.for_each(|c| { dbg!(c); });

    let lex = Lexer::new(file_content.as_str());


    let mut parser = Parser::new(lex);

    println!("{}", format!("======= ZipetteInterpreter v{VERSION} =======").on_cyan());

    match parser.parse() {
        Ok(program) => {
            Interpreter::new(program).interpret();;
        },
        Err(err) => {
            println!("{}", err.to_string().red());
        },
    };

}