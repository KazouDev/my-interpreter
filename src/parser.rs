use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use colored::Colorize;
use rand::Rng;
use crate::lexer::Token;

#[derive(Debug)]
pub enum BinaryExpressionType {
    Sum,
    Product,
    Division,
    Minus,
    Exponent,
    BytesLeft,
    BytesRight,
}

#[derive(Debug)]
pub enum Expression {
    Number(f64),
    Identifier(String),
    Binary {
        op: BinaryExpressionType,
        left: Box<Expression>,
        right: Box<Expression>
    },
}

#[derive(Debug)]
pub enum Colored {
    Red,
    Blue,
    Green,
    Yellow,
    Purple,
    Cyan,
    Orange,
    White,
    Brown,
    Pink,
    MultiColor
}

impl Colored {
    fn random() -> Self {
        let mut rng = rand::rng();
        match rng.random_range(0..=9) {
            0 => Colored::Red,
            1 => Colored::Blue,
            2 => Colored::Green,
            3 => Colored::Yellow,
            4 => Colored::Purple,
            5 => Colored::Cyan,
            6 => Colored::Orange,
            7 => Colored::White,
            8 => Colored::Brown,
            9 => Colored::Pink,
            _ => unreachable!()
        }
    }
}

#[derive(Debug)]
pub enum Statement {
    Expression(Expression),
    Print(Expression),
    PrintColored(Colored,Expression),
    Assignment(String, Expression),
}

#[derive(Debug)]
pub struct ParseError(String);

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[PARSER] Error : {}", self.0)
    }
}

impl Error for ParseError {}

pub struct Parser<I: Iterator<Item = Token>> {
    tokens: I,
    current: Option<Token>,
}

impl<I: Iterator<Item=Token>> Parser<I> {
    pub fn new(mut tokens: I) -> Self {
        let current = tokens.next();
        Self { tokens, current }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, ParseError> {
        let mut statements = Vec::new();
        while let Some(token) = &self.current {
            if let Token::Identifier(id) = token {
                statements.push(match id.as_str() {
                    "zipette" => {
                        self.consume();
                       Statement::Print(self.parse_expression())
                    },
                    "lsd" => {
                        if let Some(Token::Identifier(token)) = self.tokens.next() {
                            self.consume();
                            let color = match token.as_str() {
                                "red" => Colored::Red,
                                "blue" => Colored::Blue,
                                "green" => Colored::Green,
                                "yellow" => Colored::Yellow,
                                "multicolor" | "multi" => Colored::MultiColor,
                                _ => return Err(ParseError(format!("Unrecognised color type '{}'", token)))
                            };

                            Statement::PrintColored(color, self.parse_expression())
                        } else {
                            return Err(ParseError("Unexpected end of statement (; required)".to_string()));
                        }
                    },
                    "vicer" => {
                        if let Some(Token::Identifier(token)) = self.tokens.next() {
                            self.consume();
                            Statement::Assignment(token, self.parse_expression())
                        } else {
                            return Err(ParseError("Unexpected variable name".to_string()));
                        }
                    },
                    _ => return Err(ParseError(format!("Unexpected identifier '{}'", id)))
                });
            } else {
                statements.push(Statement::Expression(self.parse_expression()))
            }

            if !matches!(self.current, Some(Token::EndOfStatement)) {
                return Err(ParseError("Unexpected end of statement (; required)".to_string()));
            }
            self.consume();

        }
        Ok(statements)
    }

    fn consume(&mut self) {
        self.current = self.tokens.next();
    }

    pub fn parse_expression(&mut self) -> Expression {
        self.term_expression()
    }

    fn term_expression(&mut self) -> Expression {
        let mut left = self.factor_expression();
        while let Some(token) = &self.current {
            let op = match token {
                Token::Plus => BinaryExpressionType::Sum,
                Token::Minus => BinaryExpressionType::Minus,
                _ => break,
            };

            self.consume();
            left = Expression::Binary {
                op,
                left: Box::new(left),
                right: Box::new(self.factor_expression())
            };
        }
        left
    }

    fn factor_expression(&mut self) -> Expression {
        let mut left = self.exponent_expression();
        while let Some(token) = &self.current {
            let op = match token {
                Token::Product => BinaryExpressionType::Product,
                Token::Division => BinaryExpressionType::Division,
                Token::BytesLeft => BinaryExpressionType::BytesLeft,
                Token::BytesRight => BinaryExpressionType::BytesRight,
                _ => break,
            };

            self.consume();
            left = Expression::Binary {
                op,
                left: Box::new(left),
                right: Box::new(self.exponent_expression())
            };
        }
        left
    }

    fn exponent_expression(&mut self) -> Expression {
        let mut left = self.parse_literal();
        while let Some(token) = &self.current {
           match token {
               Token::Exponent => {
                   self.consume();
                   left = Expression::Binary {
                       op: BinaryExpressionType::Exponent,
                       left: Box::new(left),
                       right: Box::new(self.exponent_expression())
                   };
               },
               _ => break,
           };
        }
        left
    }

    fn parse_literal(&mut self) -> Expression {
        match self.current.take() {
            Some(Token::Number(n)) => {
                self.consume();
                Expression::Number(n)
            }
            Some(Token::OpenParen) => {
                self.consume();
                let expr = self.parse_expression();
                if let Some(Token::CloseParen) = self.current.take() {
                    self.consume();
                    expr
                } else {
                    panic!("Expected ')' at the end");
                }
            },
            Some(Token::Identifier(id)) => {
                self.consume();
                Expression::Identifier(id)
            }
            other => {
                println!("Unexpected token: {:?}", other);
                panic!("Expected a number");
            }
        }
    }
}

#[derive(Debug)]
pub struct ExecuteError(String);

impl Display for ExecuteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[EXECUTION] Error : {}", self.0)
    }
}

impl Error for ExecuteError {}

impl Expression {
    pub fn evaluate(&self, variables: &HashMap<String, f64>) -> Result<f64, ExecuteError> {
        match self {
            Expression::Identifier(id) => {
                if let Some(value) = variables.get(id) {
                    Ok(*value)
                } else {
                    Err(ExecuteError(format!("use of undefined variable {}", id)))
                }
            },
            Expression::Number(n) => Ok(*n),
            Expression::Binary { op, left, right} => {
                match op {
                    BinaryExpressionType::Sum => Ok(left.evaluate(variables)? + right.evaluate(variables)?),
                    BinaryExpressionType::Product => Ok(left.evaluate(variables)? * right.evaluate(variables)?),
                    BinaryExpressionType::Division => Ok(left.evaluate(variables)? / right.evaluate(variables)?),
                    BinaryExpressionType::Minus => Ok(left.evaluate(variables)? - right.evaluate(variables)?),
                    BinaryExpressionType::Exponent => Ok(left.evaluate(variables)?.powf(right.evaluate(variables)?)),
                    BinaryExpressionType::BytesLeft => Ok((left.evaluate(variables)?.trunc() as u64).checked_shl(right.evaluate(variables)? as u32).unwrap_or(0) as f64),
                    BinaryExpressionType::BytesRight => Ok((left.evaluate(variables)?.trunc() as u64).checked_shr(right.evaluate(variables)? as u32).unwrap_or(0) as f64),
                }
            }
        }
    }
}

impl Statement {
    pub fn execute(self, variables: &mut HashMap<String, f64>) -> Result<(), ExecuteError> {
        match self {
            Statement::Expression(expr) => expr.evaluate(variables).map(|_| ())?,
            Statement::Print(expr) => println!("{}", expr.evaluate(variables)?),
            Statement::Assignment(lhs, rhs) => {
                variables.insert(lhs, rhs.evaluate(variables)?);
            }
            Statement::PrintColored(color, expr) => {
                let value = format!("{}", expr.evaluate(variables)?);

                match color {
                    Colored::Red => println!("{}", value.red()),
                    Colored::Blue => println!("{}", value.blue()),
                    Colored::Yellow => println!("{}", value.yellow()),
                    Colored::Green => println!("{}", value.green()),
                    Colored::Purple => println!("{}", value.purple()),
                    Colored::Cyan => println!("{}", value.cyan()),
                    Colored::Orange => println!("{}", value.custom_color((255,127,0))),
                    Colored::White => println!("{}", value.white()),
                    Colored::Brown => println!("{}", value.custom_color((165,42,42))),
                    Colored::Pink => println!("{}", value.custom_color((255,20,147))),
                    Colored::MultiColor => {
                        value.split("")
                            .for_each(|x| {
                                match Colored::random() {
                                    Colored::Red => print!("{}", x.red()),
                                    Colored::Blue => print!("{}", x.blue()),
                                    Colored::Yellow => print!("{}", x.yellow()),
                                    Colored::Green => print!("{}", x.green()),
                                    Colored::Purple => print!("{}", x.purple()),
                                    Colored::Cyan => print!("{}", x.cyan()),
                                    Colored::Orange => print!("{}", x.custom_color((255,127,0))),
                                    Colored::White => print!("{}", x.white()),
                                    Colored::Brown => print!("{}", x.custom_color((165,42,42))),
                                    Colored::Pink => print!("{}", x.custom_color((255,20,147))),
                                    _ => unreachable!()
                                }
                            });
                        println!();
                    }
                }
            }
        };
        Ok(())
    }
}