use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum Token {
    Number(f64),
    EndOfStatement,
    Identifier(String),
    Minus,
    Plus,
    Product,
    Division,
    Exponent,
    OpenParen,
    CloseParen,
    Useless(char),
    Bad(LexerError),
    BytesLeft,
    BytesRight,
}

pub struct Location {
    line: usize,
    start_column: usize,
    end_column: usize
}

pub struct LocalizedToken {
    pub token: Token,
    pub loc: Location
}

#[derive(Debug)]
pub struct LexerError(String);

impl Display for LexerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "LexerError: {}", self.0)
    }
}

impl Error for LexerError {}

pub struct Lexer<'a> {
    input: &'a str,
    cursor: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, cursor: 0 }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        let c = self.peek_char()?;

        let token = match c {
            '-' => {
                self.consume();
                if let Some(next) = self.peek_char() {
                    if next.is_ascii_digit() {
                        return Some(self.parse_number(true));
                    }
                }
                Token::Minus
            }
            '+' => {
                self.consume();
                Token::Plus
            }
            '*' => {
                self.consume();
                if self.peek_char() == Some('*') {
                    self.consume();
                    Token::Exponent
                } else {
                    Token::Product
                }
            }
            '^' => {
                self.consume();
                Token::Exponent
            },
            '(' => {
                self.consume();
                Token::OpenParen
            },
            ')' => {
                self.consume();
                Token::CloseParen
            },
            ';' => {
                self.consume();
                Token::EndOfStatement
            },
            '/' => {
                self.consume();
                Token::Division
            },
            '<' => {
                if let Some(ch) = self.peek_char() {
                    if ch == '<' {
                        Token::BytesLeft
                    } else {
                        Token::Useless(ch)
                    }
                } else {
                    Token::Useless('<')
                }
            },
            '>' => {
                self.consume();
                if let Some(ch) = self.peek_char() {
                    if ch == '>' {
                        self.consume();
                        Token::BytesRight
                    } else {
                        Token::Useless('>')
                    }
                } else {
                    Token::Useless('>')
                }
            },
            '0'..='9' => self.parse_number(false),
            'a'..='z' | 'A'..='Z' => self.parse_identifier(),
            _ => {
                self.consume();
                Token::Useless(c)
            }
        };

        Some(token)
    }

    fn parse_number(&mut self, is_negative: bool) -> Token {
        let mut num_str = self.consume_while(|c| c.is_ascii_digit());

        if self.peek_char() == Some('.') || self.peek_char() == Some(',') {
            self.consume();
            num_str += ".";
            num_str += self.consume_while(|r#c| c.is_ascii_digit()).as_str();
        }

        match num_str.parse::<f64>() {
            Ok(n) => Token::Number(if is_negative { -n } else { n }),
            Err(_) => Token::Bad(LexerError(format!("Invalid number: {}", num_str))),
        }
    }

    fn skip_whitespace(&mut self) {
        self.consume_while(|c| c.is_whitespace());
    }

    fn parse_identifier(&mut self) -> Token {
        Token::Identifier(self.consume_while(|c| c.is_ascii_alphabetic() && c != ';'))
    }

    fn consume(&mut self) -> Option<char> {
        if let Some(c) = self.input[self.cursor..].chars().next() {
            self.cursor += c.len_utf8();
            Some(c)
        } else {
            None
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.input[self.cursor..].chars().next()
    }

    fn consume_while<F>(&mut self, condition: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let start = self.cursor;
        while let Some(c) = self.peek_char() {
            if !condition(c) {
                break;
            }
            self.consume();
        }
        self.input[start..self.cursor].to_string()
    }
}

impl<> Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
