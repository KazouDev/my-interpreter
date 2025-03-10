use crate::parser::Expression;

pub struct Interpreter {
    expr: Expression,
}

impl Interpreter {
    pub fn new(expr: Expression) -> Self {
        Self { expr }
    }

    pub fn interpret(&self) -> i64 {
        self.expr.evaluate()
    }
}
