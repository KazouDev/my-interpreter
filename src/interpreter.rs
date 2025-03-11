use std::collections::HashMap;
use colored::Colorize;
use crate::parser::{Statement};

pub struct Interpreter {
    program: Vec<Statement>,
    variables: HashMap<String, f64>,
}

impl Interpreter {
    pub fn new(program: Vec<Statement>) -> Self {
        Self { program, variables: HashMap::new() }
    }

    pub fn show(&self) {
        for stmt in &self.program {
            dbg!(stmt);
        }
    }

    pub fn interpret(mut self) -> f64 {
        let _ = self.
            program.
            into_iter().
            try_for_each(|c|{
                c.execute(&mut self.variables)
            })
            .inspect_err(|err|{
                println!("{}", err.to_string().red());
            });
        1.0
    }
}
