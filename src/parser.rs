use std::collections::HashMap;
use std::env::var;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::lexer::Token;

#[derive(Debug)]
pub enum BinaryExpressionType {
    Sum,
    Product,
    Minus,
    Exponent,
}

#[derive(Debug)]
pub enum Expression {
    Number(f64),
    Identifier(String),
    BinaryExpression {
        op: BinaryExpressionType,
        left: Box<Expression>,
        right: Box<Expression>
    }
}

#[derive(Debug)]
pub enum Statement {
    Expression(Expression),
    Print(Expression),
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
                    "vicer" => {
                        if let Some(Token::Identifier(token)) = self.tokens.next() {
                            self.consume();
                            Statement::Assignment(token, self.parse_expression())
                        } else {
                            return Err(ParseError("Unexpected end of statement (; required)".to_string()));
                        }
                    },
                    _ => return Err(ParseError(format!("Expected identifier, found '{}'", id)))
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
            left = Expression::BinaryExpression {
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
                _ => break,
            };

            self.consume();
            left = Expression::BinaryExpression {
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
                   left = Expression::BinaryExpression {
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
                    Ok(value.clone())
                } else {
                    Err(ExecuteError(format!("use of undefined variable {}", id)))
                }
            },
            Expression::Number(n) => Ok(*n),
            Expression::BinaryExpression { op, left, right} => {
                match op {
                    BinaryExpressionType::Sum => Ok(left.evaluate(variables)? + right.evaluate(variables)?),
                    BinaryExpressionType::Product => Ok(left.evaluate(variables)? * right.evaluate(variables)?),
                    BinaryExpressionType::Minus => Ok(left.evaluate(variables)? - right.evaluate(variables)?),
                    BinaryExpressionType::Exponent => Ok(left.evaluate(variables)?.powf(right.evaluate(variables)?)),
                }
            }
        }
    }
}

impl Statement {
    pub fn execute(self, variables: &mut HashMap<String, f64>) -> Result<(), ExecuteError> {
        Ok(match self {
            Statement::Expression(expr) => expr.evaluate(variables).map(|_| ())?,
            Statement::Print(expr) => println!("{}", expr.evaluate(variables)?),
            Statement::Assignment(lhs, rhs) => {
                variables.insert(lhs, rhs.evaluate(variables)?);
            }
        })
    }
}