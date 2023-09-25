use self::token::{Literal, Token, TokenType};
use super::*;
use std::collections::HashMap;
use std::process;

pub mod token;

pub struct Scanner {
    source: Vec<u8>,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    pub line: usize,
    keywords: HashMap<&'static str, TokenType>,
}

impl Scanner {
    pub fn new(source: String, line: usize) -> Self {
        let source: Vec<u8> = source.bytes().collect();
        let mut keywords = HashMap::new();
        keywords.insert("and", TokenType::And);
        keywords.insert("class", TokenType::Class);
        keywords.insert("else", TokenType::Else);
        keywords.insert("false", TokenType::False);
        keywords.insert("for", TokenType::For);
        keywords.insert("fun", TokenType::Fun);
        keywords.insert("if", TokenType::If);
        keywords.insert("nil", TokenType::Nil);
        keywords.insert("or", TokenType::Or);
        keywords.insert("print", TokenType::Print);
        keywords.insert("return", TokenType::Return);
        keywords.insert("super", TokenType::Super);
        keywords.insert("this", TokenType::This);
        keywords.insert("true", TokenType::True);
        keywords.insert("var", TokenType::Var);
        keywords.insert("while", TokenType::While);

        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line,
            keywords,
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
                    } else if self.match_token('*') {
                        self.block_quote();
                    } else {
                        self.add_token(TokenType::Slash, None);
                    }
                }
                ' ' | '\r' | '\t' => {}
                '\n' => {
                    self.line += 1;
                }
                '"' => self.string(),
                _ => {
                    if c.is_digit(10) {
                        self.number();
                    } else if c.is_alphabetic() || c == '_' {
                        self.identifier();
                    } else {
                        self.make_error(self.line, "Unexpected character.");
                    }
                }
            }
        }
    }
    fn make_error(&self, line: usize, message: &str) {
        unsafe {
            GLOBAL_LOX.error(line, message);
        }
    }

    fn block_quote(&mut self) {
        while self.is_not_at_end() && !(self.peek() == '*' && self.peek_next() == '/') {
            self.advance();
        }

        if self.is_at_end() {
            self.make_error(self.line, "Unterminated block quote");
        }

        self.advance();
        self.advance();
    }

    fn is_not_at_end(&self) -> bool {
        return !self.is_at_end();
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let text = String::from_utf8(self.source[self.start..self.current].to_owned())
            .unwrap_or_else(|err| {
                eprint!("{err}");
                process::exit(64)
            });

        if let Some(a) = self.keywords.get(&text[..]) {
            self.add_token(a.to_owned(), None);
        } else {
            self.add_token(TokenType::Identifier, None);
        }
    }
    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let literal: f64 = String::from_utf8(self.source[self.start..self.current].to_owned())
            .unwrap_or_else(|err| {
                eprintln!("{err}");
                process::exit(65);
            })
            .parse()
            .unwrap_or_else(|err| {
                eprintln!("{err}");
                process::exit(65);
            });
        self.add_token(TokenType::Number, Some(Literal::Number(literal)))
    }

    fn peek_next(&self) -> char {
        if self.current + 1 > self.source.len() {
            return '\0';
        }
        return char::from(self.source[self.current + 1].clone());
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.make_error(self.line, "Unterminated string.");
            return;
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

    /// Tries to match the expected character to the current character. It returns false if alread at the end of the source code.
    ///
    /// # Arguments
    ///
    /// * `expected` - The character to be matched
    ///
    /// # Returns
    ///
    /// true if `expected` matches the current character else false.
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

    /// Advances the current character to the next and returns it.
    /// 
    /// ### Returns
    /// 
    /// The new current character wrapped in Some or None if alread at end.
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
