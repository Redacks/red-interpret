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
    pub fn new(input: &String) -> Self {
        Lexer {
            input: input.to_owned(),
            current: 0,
            start: 0,
            line: 1,
            tokens: Vec::new(),
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, CodeError> {
        if !self.is_at_end() {
            self.op_token()?;
            while !self.is_at_end() {
                self.start = self.current;
                self.scan_token()?
            }
            self.add_token(TokenType::EOF);
            Ok(self.tokens.clone())
        } else {
            Ok(self.tokens.clone()) //Empty
        }
    }

    fn scan_token(&mut self) -> Result<(), CodeError> {
        match self.tokens.last().unwrap().token_type {
            TokenType::NEWLINE => {
                match self.advance()? {
                    '\n' => {
                        self.add_token(TokenType::NEWLINE);
                        self.line += 1;
                        self.start = self.current;
                    }
                    ' ' => {
                        self.start = self.current;
                    }
                    _ => {
                        self.current -= 1;
                    }
                }
                self.op_token()?;
                Ok(())
            }
            _ => match self.advance()? {
                '\n' => {
                    self.add_token(TokenType::NEWLINE);
                    self.line += 1;
                    self.start = self.current;
                    Ok(())
                }
                ' ' => {
                    self.start = self.current;
                    Ok(())
                }
                _ => Err(CodeError::new(
                    self.line,
                    self.start,
                    self.current,
                    "Unknown token at this position!",
                )),
            },
        }
    }

    fn identifier_token(&mut self) -> Result<(), CodeError> {
        let mut ch = self.advance()?;
        while ch.is_alphanumeric() || ch == '_' {
            ch = self.advance()?;
        }
        self.current -= 1;
        self.add_token(TokenType::IDENTIFIER);
        Ok(())
    }

    fn string_token(&mut self) -> Result<(), CodeError> {
        self.skip_spaces()?;
        let mut ch;
        loop {
            ch = self.advance()?;
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
    fn number_token(&mut self) -> Result<(), CodeError> {
        self.skip_spaces()?;
        let mut ch = self.advance()?;
        while ch.is_digit(10) {
            ch = self.advance()?;
        }
        self.current -= 1;
        self.add_token(TokenType::VALUE);
        Ok(())
    }

    fn equal_token(&mut self) -> Result<(), CodeError> {
        self.skip_spaces()?;
        if self.advance()? == '=' {
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
        match self.advance_space()?.as_str() {
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
        let ch = self.advance()?;
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
            slice,
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
                "EOF reached to early!",
            ))
        }
    }

    fn skip_spaces(&mut self) -> Result<(), CodeError> {
        let mut ch = self.advance()?;
        while ch == ' ' {
            ch = self.advance()?;
        }
        self.current -= 1;
        self.start = self.current;
        Ok(())
    }

    fn advance_space(&mut self) -> Result<String, CodeError> {
        let mut result: String = String::new();
        let mut char = self.advance()?;
        while char != ' ' {
            result.push(char);
            char = self.advance()?;
        }
        Ok(result)
    }

    fn get_char_at_current(&self) -> Result<char, CodeError> {
        if let Some(char) = self.input.chars().nth(self.current) {
            Ok(char)
        } else {
            Err(CodeError::new(
                self.line,
                self.start,
                self.current,
                "Error while parsing line!",
            ))
        }
    }

    fn advance(&mut self) -> Result<char, CodeError> {
        let char = self.get_char_at_current();
        self.current += 1;
        char
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.input.len();
    }
}
