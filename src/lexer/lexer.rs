use std::borrow::Cow;

use crate::error::CodeError;

use super::{Token, TokenType};

pub struct Lexer {
    input: String,
    current: usize,
    start: usize,
    line: usize,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.to_owned(),
            current: 0,
            start: 0,
            line: 1,
            tokens: Vec::new(),
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, CodeError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }
        self.add_token(TokenType::EOF);
        Ok(self.tokens.to_owned())
    }

    fn scan_token(&mut self) -> Result<(), CodeError> {
        if let Some(last_token) = self.tokens.last() {
            match last_token.token_type {
                TokenType::NEWLINE => {
                    self.op_token()?;
                    Ok(())
                }
                _ => match self.advance("Invalid Instruction")? {
                    '\n' => {
                        self.add_token(TokenType::NEWLINE);
                        self.line += 1;
                        Ok(())
                    }
                    ' ' => {
                        self.start = self.current;
                        Ok(())
                    }
                    _ => {
                        return Err(CodeError::new(
                            self.line,
                            self.start,
                            self.current,
                            "Unknown token at this position!",
                        ))
                    }
                },
            }
        } else {
            self.op_token()?;
            Ok(())
        }
    }

    fn identifier_token(&mut self) -> Result<(), CodeError> {
        let mut ch = self.advance("Expected identifier name")?;
        while (ch.is_alphanumeric() || ch == '_') && !self.is_at_end() {
            ch = self.advance("Unexpected end of identifier")?;
        }
        self.current -= 1;
        self.add_token(TokenType::IDENTIFIER);
        Ok(())
    }

    fn string_token(&mut self) -> Result<(), CodeError> {
        self.skip_spaces()?;
        while !self.is_at_end() {
            let ch = self.advance("Unexpected end of string")?;
            match ch {
                '$' => {
                    if self.match_next('$')? {
                        self.current -= 1;
                        self.add_token(TokenType::VALUE);
                        self.start = self.current + 1;
                        self.current = self.start;
                        continue;
                    } else {
                        self.current -= 1;
                        self.add_token(TokenType::VALUE);
                        self.start = self.current + 1;
                        self.current = self.start;

                        self.identifier_token()?;
                        if !self.match_next('$')? {
                            return Err(CodeError::new(
                                self.line,
                                self.start,
                                self.current,
                                "Expected closing $ for variable",
                            ));
                        } else if self.match_next('$')? {
                            return Err(CodeError::new(
                                self.line,
                                self.start,
                                self.current,
                                "Found $$ but expected $ because of present variable",
                            ));
                        } else {
                            self.start = self.current;
                            continue;
                        }
                    }
                }
                '\n' => {
                    self.current -= 1;
                    break;
                }
                _ => continue,
            }
        }
        self.add_token(TokenType::VALUE);
        Ok(())
    }

    fn math_token(&mut self) -> Result<(), CodeError> {
        self.skip_spaces()?;
        if self.is_at_end() || self.tokens.last().unwrap().token_type == TokenType::NEWLINE {
            return Ok(());
        }
        match self.advance("Expected Math expression")? {
            '+' => self.add_token(TokenType::ADD),
            '-' => self.add_token(TokenType::SUB),
            '*' => self.add_token(TokenType::MULT),
            '/' => self.add_token(TokenType::DIV),
            _ => {
                return Err(CodeError::new(
                    self.line,
                    self.start,
                    self.current,
                    "Expected Math Operator (+,-,*,/)",
                ))
            }
        }
        self.number_token()?;
        Ok(())
    }

    fn number_token(&mut self) -> Result<(), CodeError> {
        self.skip_spaces()?;
        if self.is_at_end() || self.tokens.last().unwrap().token_type == TokenType::NEWLINE {
            return Ok(());
        }
        let mut ch = self.advance("Expected Number value")?;

        if ch.is_digit(10) {
            // Numeric value handling
            while ch.is_digit(10) && !self.is_at_end() {
                ch = self.advance("Unexpected ending of Number")?;
            }
            self.current -= 1;
            self.add_token(TokenType::VALUE);
        } else if ch == '$' {
            self.start = self.current;
            self.identifier_token()?;
            if !self.match_next('$')? {
                return Err(CodeError::new(
                    self.line,
                    self.start,
                    self.current,
                    "Expected closing $ for variable",
                ));
            } else if self.match_next('$')? {
                return Err(CodeError::new(
                    self.line,
                    self.start,
                    self.current,
                    "Found $$ but expected $ because of present variable",
                ));
            }
        } else {
            return Err(CodeError::new(
                self.line,
                self.start,
                self.current,
                "Expected digit or variable in number token",
            ));
        }
        self.math_token()?;
        Ok(())
    }

    fn equal_token(&mut self) -> Result<(), CodeError> {
        self.skip_spaces()?;
        if self.advance("Expected Assignment with '=' ")? == '=' {
            self.add_token(TokenType::EQUAL);
            Ok(())
        } else {
            Err(CodeError::new(
                self.line,
                self.start,
                self.current,
                "Expected Assignment with '=' ",
            ))
        }
    }

    fn op_token(&mut self) -> Result<(), CodeError> {
        self.skip_spaces()?;
        if self.is_at_end() {
            return Ok(());
        }
        match self.advance_space()?.as_ref() {
            "Zahl" => {
                self.add_token(TokenType::ZAHL);
                self.identifier_token()?;
                if self.is_assignment()? {
                    self.equal_token()?;
                    self.number_token()?;
                }
                Ok(())
            }
            "Text" => {
                self.add_token(TokenType::TEXT);
                self.identifier_token()?;
                if self.is_assignment()? {
                    self.equal_token()?;
                    self.string_token()?;
                }
                Ok(())
            }
            "Input" => {
                self.add_token(TokenType::INPUT);
                self.identifier_token()?;
                Ok(())
            }
            "Output" => {
                self.add_token(TokenType::OUTPUT);
                self.identifier_token()?;
                Ok(())
            }
            _ => Err(CodeError::new(
                self.line,
                self.start,
                self.current,
                "Expected Text, Zahl, Output or Input!",
            )),
        }
    }

    fn is_assignment(&mut self) -> Result<bool, CodeError> {
        self.skip_spaces()?;
        if self.tokens.last().unwrap().token_type == TokenType::NEWLINE {
            return Ok(false);
        }
        let ch = self.advance("Unexpected ending of Assignment")?;
        self.current = self.start;
        Ok(ch == '=')
    }

    fn add_token(&mut self, token_type: TokenType) {
        let slice: String = self
            .input
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect();
        self.tokens.push(Token::new(
            self.line,
            self.start,
            self.current,
            slice.to_owned(),
            token_type,
        ));
        self.start = self.current;
    }

    fn match_next(&mut self, expected: char) -> Result<bool, CodeError> {
        if self.is_at_end() {
            Ok(false)
        } else if let Some(char) = self.input.chars().nth(self.current) {
            if char != expected {
                Ok(false)
            } else {
                self.current += 1;
                Ok(true)
            }
        } else {
            Err(CodeError::new(
                self.line,
                self.start,
                self.current,
                "EOF reached too early!",
            ))
        }
    }

    fn skip_spaces(&mut self) -> Result<(), CodeError> {
        if self.is_at_end() {
            return Ok(());
        }
        let mut ch = self.advance("Unexpected end of file!")?;
        while ch.is_whitespace() && !self.is_at_end() {
            if ch == '\n' {
                self.add_token(TokenType::NEWLINE);
                self.line += 1;
            }
            ch = self.advance("Unexpected end of file!")?;
        }
        if !self.is_at_end() {
            self.current -= 1;
            self.start = self.current;
        }
        Ok(())
    }

    fn advance_space(&mut self) -> Result<Cow<str>, CodeError> {
        while let Ok(char) = self.advance("Expected Space at the end!") {
            if char == ' ' {
                break;
            }
        }
        let slice: String = self
            .input
            .chars()
            .skip(self.start)
            .take(self.current - 1 - self.start)
            .collect();
        Ok(slice.into())
    }

    fn get_char_at_current(&self, err_msg: &str) -> Result<char, CodeError> {
        if let Some(char) = self.input.chars().nth(self.current) {
            Ok(char)
        } else {
            Err(CodeError::new(self.line, self.start, self.current, err_msg))
        }
    }

    fn advance(&mut self, err_msg: &str) -> Result<char, CodeError> {
        let char = self.get_char_at_current(err_msg)?;
        self.current += 1;
        Ok(char)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.input.chars().count()
    }
}
