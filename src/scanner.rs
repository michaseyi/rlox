use self::token::{Literal, Token, TokenType};
use super::*;
use std::process;

pub mod token;
pub struct Scanner {
    source: Vec<u8>,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    pub line: usize,
}

impl Scanner {
    pub fn new(source: String, line: usize) -> Self {
        let source: Vec<u8> = source.bytes().collect();
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line,
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(
            token::TokenType::Eof,
            String::from(""),
            self.line,
            None,
        ));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    fn scan_token(&mut self) {
        let c = self.advance();

        if let Some(c) = c {
            match c {
                '(' => {
                    self.add_token(TokenType::LeftParen, None);
                }
                ')' => {
                    self.add_token(TokenType::RightParen, None);
                }
                '{' => {
                    self.add_token(TokenType::LeftBrace, None);
                }
                '}' => {
                    self.add_token(TokenType::RightBrace, None);
                }
                ',' => {
                    self.add_token(TokenType::Comma, None);
                }
                '.' => {
                    self.add_token(TokenType::Dot, None);
                }
                '-' => {
                    self.add_token(TokenType::Minus, None);
                }
                '+' => {
                    self.add_token(TokenType::Plus, None);
                }
                ';' => {
                    self.add_token(TokenType::SemiColon, None);
                }
                '*' => {
                    self.add_token(TokenType::Star, None);
                }
                '!' => {
                    if self.match_token('=') {
                        self.add_token(TokenType::BangEqual, None)
                    } else {
                        self.add_token(TokenType::Bang, None)
                    }
                }
                '=' => {
                    if self.match_token('=') {
                        self.add_token(TokenType::EqualEqual, None)
                    } else {
                        self.add_token(TokenType::Equal, None)
                    }
                }
                '<' => {
                    if self.match_token('=') {
                        self.add_token(TokenType::LessEqual, None)
                    } else {
                        self.add_token(TokenType::Less, None)
                    }
                }
                '>' => {
                    if self.match_token('=') {
                        self.add_token(TokenType::GreaterEqual, None)
                    } else {
                        self.add_token(TokenType::Greater, None)
                    }
                }
                '/' => {
                    if self.match_token('/') {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        self.add_token(TokenType::Slash, None);
                    }
                }
                ' ' | '\r' | '\t' => {}
                '\n' => {
                    self.line += 1;
                }
                '"' => self.string(),
                _ => unsafe {
                    GLOBAL_LOX.error(self.line, "Unexpected character.");
                },
            }
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            unsafe {
                GLOBAL_LOX.error(self.line, "Unterminated string.");
                return;
            }
        }

        self.advance();
        let literal = String::from_utf8(self.source[self.start + 1..self.current - 1].to_owned())
            .unwrap_or_else(|err| {
                eprintln!("{err}");
                process::exit(65);
            });
        self.add_token(TokenType::String, Some(Literal::String(literal)));
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        char::from(self.source[self.current])
    }
    fn match_token(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if char::from(self.source[self.current]) != expected {
            return false;
        }
        self.current += 1;
        return true;
    }
    fn advance(&mut self) -> Option<char> {
        let i = self.source.get(self.current)?;
        self.current += 1;
        Some(char::from(i.to_owned()))
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let lexeme = String::from_utf8(self.source[self.start..self.current].to_owned())
            .unwrap_or_else(|err| {
                eprint!("{err}");
                process::exit(64)
            });
        self.tokens
            .push(Token::new(token_type, lexeme, self.line, literal))
    }
}
