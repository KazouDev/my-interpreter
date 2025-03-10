use crate::lexer::Token;

#[derive(Debug)]
enum BinaryExpressionType {
    Sum,
    Product,
    Minus,
    Exponent,
}

#[derive(Debug)]
pub enum Expression {
    Number(i64),
    BinaryExpression {
        op: BinaryExpressionType,
        left: Box<Expression>,
        right: Box<Expression>
    }
}

pub struct Parser<I: Iterator<Item = Token>> {
    tokens: I,
    current: Option<Token>,
}

impl<I: Iterator<Item=Token>> Parser<I> {
    pub fn new(mut tokens: I) -> Self {
        let current = tokens.next();
        Self { tokens, current }
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
            }
            other => {
                println!("Unexpected token: {:?}", other);
                panic!("Expected a number");
            }
        }
    }
}

impl Expression {
    pub fn evaluate(&self) -> i64 {
        match self {
            Expression::Number(n) => *n,
            Expression::BinaryExpression { op, left, right} => {
                match op {
                    BinaryExpressionType::Sum => left.evaluate() + right.evaluate(),
                    BinaryExpressionType::Product => left.evaluate() * right.evaluate(),
                    BinaryExpressionType::Minus => left.evaluate() - right.evaluate(),
                    BinaryExpressionType::Exponent => left.evaluate().pow(right.evaluate().try_into().expect("exponent overflow (u32)")),
                }
            }
        }
    }
}