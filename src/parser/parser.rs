use std::backtrace::Backtrace;

use crate::lexer::{Token, TokenType};

use super::{
    Expression, ExpressionTypes, NumberExpression, NumberExpressionTypes, TextExpression,
    TextExpressionTypes,
};

pub struct Parser {
    tokens: Vec<Token>,
    token_idx: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            token_idx: 0,
        }
    }

    fn next_token(&mut self) {
        self.token_idx += 1;
    }

    fn get_current_token(&self) -> Result<Token, String> {
        if let Some(token) = self.tokens.get(self.token_idx) {
            Ok(token.to_owned())
        } else {
            Err(format!(
                "Error while parsing! EOF {}",
                Backtrace::force_capture()
            ))
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Expression>, String> {
        let mut expressions = Vec::new();
        while self.token_idx < self.tokens.len() {
            let token = self.get_current_token()?;
            match token.token_type {
                TokenType::NEWLINE => {
                    self.next_token();
                    continue;
                }
                _ => {
                    expressions.push(self.parser_instruction()?);
                    self.next_token()
                }
            }
        }
        Ok(expressions)
    }

    fn parser_instruction(&mut self) -> Result<Expression, String> {
        if let Some(token) = self.tokens.get(self.token_idx) {
            match token.token_type {
                TokenType::TEXT => {
                    let (identifier, value, line) = self.try_get_string_assignment()?;
                    Ok(Expression::new(
                        line,
                        ExpressionTypes::TextAssignment(identifier, value),
                    ))
                }
                TokenType::ZAHL => {
                    let (identifier, value, line) = self.try_get_number_assignment()?;
                    Ok(Expression::new(
                        line,
                        ExpressionTypes::NumberAssignment(identifier, value),
                    ))
                }
                TokenType::OUTPUT => {
                    self.next_token();
                    let identifier = self.try_get_identifier()?;
                    Ok(Expression::new(
                        self.get_current_token()?.line,
                        ExpressionTypes::OutputStatement(identifier),
                    ))
                }
                TokenType::INPUT => {
                    self.next_token();
                    let identifier = self.try_get_identifier()?;
                    Ok(Expression::new(
                        self.get_current_token()?.line,
                        ExpressionTypes::InputStatement(identifier),
                    ))
                }
                _ => Err(format!(
                    "Invalid instruction on line {}",
                    self.get_current_token()?.line
                )),
            }
        } else {
            Err(format!("Instruction parsing error!"))
        }
    }

    fn try_get_string_assignment(&mut self) -> Result<(String, TextExpression, usize), String> {
        let line = self.get_current_token()?.line;

        self.next_token();
        let identifier = self.try_get_identifier()?;

        self.next_token();
        if self.is_assignment()? {
            self.next_token();
            let value_exp = self.try_get_string_value()?;

            Ok((identifier, value_exp, line))
        } else {
            Ok((
                identifier,
                TextExpression::new(line, TextExpressionTypes::Value("".to_owned())),
                line,
            ))
        }
    }

    fn try_get_number_assignment(&mut self) -> Result<(String, NumberExpression, usize), String> {
        let line = self.get_current_token()?.line;

        self.next_token();
        let identifier = self.try_get_identifier()?;
        self.next_token();

        if self.is_assignment()? {
            self.next_token();
            let value_exp = self.try_get_number_value()?;

            Ok((identifier, value_exp, line))
        } else {
            Ok((
                identifier,
                NumberExpression::new(line, NumberExpressionTypes::Value(0)),
                line,
            ))
        }
    }

    fn try_get_identifier(&self) -> Result<String, String> {
        let token = self.get_current_token()?;
        match token.token_type {
            TokenType::IDENTIFIER(identifier) => Ok(identifier),
            _ => Err(format!("Expected identifier")),
        }
    }

    fn is_assignment(&self) -> Result<bool, String> {
        let token = self.get_current_token()?;
        match token.token_type {
            TokenType::EQUAL => Ok(true),
            TokenType::NEWLINE => Ok(false),
            _ => Err(format!("Invalid assignment on line ")),
        }
    }

    fn try_get_string_value(&mut self) -> Result<TextExpression, String> {
        let mut exp_concat = Vec::new();

        while match self.get_current_token()?.token_type {
            TokenType::VALUE(_) | TokenType::IDENTIFIER(_) => {
                exp_concat.push(TextExpression::from_token(self.get_current_token()?)?);
                self.next_token();
                self.token_idx < self.tokens.len()
            }
            _ => false,
        } {}

        if exp_concat.is_empty() {
            Err(format!(
                "Expected string value on line {}",
                self.get_current_token()?.line
            ))
        } else if exp_concat.len() == 1 {
            Ok(exp_concat.pop().unwrap())
        } else {
            let mut resulting_exp = exp_concat.pop().unwrap();
            while let Some(expr) = exp_concat.pop() {
                resulting_exp = TextExpression::join(expr, resulting_exp);
            }
            Ok(resulting_exp)
        }
    }

    fn try_get_number_value(&self) -> Result<NumberExpression, String> {
        NumberExpression::from_token(self.get_current_token()?)
    }
}
