use crate::{
    error::CodeError,
    lexer::{Token, TokenType},
};

use super::{
    Expression, ExpressionTypes, IdentifierExpression, NumberExpression, NumberExpressionTypes,
    TextExpression, TextExpressionTypes,
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

    fn get_current_token(&self) -> Result<&Token, CodeError> {
        if let Some(token) = self.tokens.get(self.token_idx) {
            Ok(token)
        } else {
            if self.tokens.len() != 0 {
                let last_token = self.tokens.last().unwrap();
                Err(CodeError::new(
                    last_token.line,
                    last_token.start,
                    last_token.end,
                    "Unknown error while parsing. EOF expected!",
                ))
            } else {
                Err(CodeError::new(0, 0, 0, "Unknown error while parsing."))
            }
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Expression>, CodeError> {
        let mut expressions = Vec::new();
        while self.token_idx < self.tokens.len() {
            let token = self.get_current_token()?;
            match token.token_type {
                TokenType::NEWLINE => {
                    self.next_token();
                    continue;
                }
                TokenType::EOF => {
                    break;
                }
                _ => {
                    expressions.push(self.parser_instruction()?);
                    self.next_token()
                }
            }
        }
        Ok(expressions)
    }

    fn parser_instruction(&mut self) -> Result<Expression, CodeError> {
        let token = self.get_current_token()?;
        let line = token.line;
        let token_start = token.start;
        match token.token_type {
            TokenType::TEXT => {
                let (identifier, value) = self.try_get_string_assignment()?;
                Ok(Expression::new(
                    line,
                    token_start,
                    value.end,
                    ExpressionTypes::TextAssignment(identifier, value),
                ))
            }
            TokenType::ZAHL => {
                let (identifier, value) = self.try_get_number_assignment()?;
                Ok(Expression::new(
                    line,
                    token_start,
                    value.end,
                    ExpressionTypes::NumberAssignment(identifier, value),
                ))
            }
            TokenType::OUTPUT => {
                self.next_token();
                let identifier = self.try_get_identifier()?;
                Ok(Expression::new(
                    line,
                    token_start,
                    identifier.end,
                    ExpressionTypes::OutputStatement(identifier),
                ))
            }
            TokenType::INPUT => {
                self.next_token();
                let identifier = self.try_get_identifier()?;
                Ok(Expression::new(
                    line,
                    token_start,
                    identifier.end,
                    ExpressionTypes::InputStatement(identifier),
                ))
            }
            _ => Err(CodeError::new(
                token.line,
                token.start,
                token.end,
                "Invalid instruction",
            )),
        }
    }

    fn try_get_string_assignment(
        &mut self,
    ) -> Result<(IdentifierExpression, TextExpression), CodeError> {
        let token = self.get_current_token()?;
        let line = token.line;
        let token_start = token.start;
        let token_end = token.end;

        self.next_token();
        let identifier = self.try_get_identifier()?;

        self.next_token();
        if self.is_assignment()? {
            self.next_token();
            let value_exp = self.try_get_string_value()?;

            Ok((identifier, value_exp))
        } else {
            Ok((
                identifier,
                TextExpression::new(
                    line,
                    token_start,
                    token_end,
                    TextExpressionTypes::Value("".to_owned()),
                ),
            ))
        }
    }

    fn try_get_number_assignment(
        &mut self,
    ) -> Result<(IdentifierExpression, NumberExpression), CodeError> {
        let token = self.get_current_token()?;
        let line = token.line;
        let token_start = token.start;
        let token_end = token.end;

        self.next_token();
        let identifier = self.try_get_identifier()?;
        self.next_token();

        if self.is_assignment()? {
            self.next_token();
            let value_exp = self.try_get_number_value()?;

            Ok((identifier, value_exp))
        } else {
            Ok((
                identifier,
                NumberExpression::new(
                    line,
                    token_start,
                    token_end,
                    NumberExpressionTypes::Value(0),
                ),
            ))
        }
    }

    fn try_get_identifier(&self) -> Result<IdentifierExpression, CodeError> {
        let token = self.get_current_token()?;
        match token.token_type {
            TokenType::IDENTIFIER => Ok(IdentifierExpression::new(
                token.line,
                token.start,
                token.end,
                token.value.to_owned(),
            )),
            _ => Err(CodeError::new(
                token.line,
                token.start,
                token.end,
                "Expected identifier",
            )),
        }
    }

    fn is_assignment(&self) -> Result<bool, CodeError> {
        let token = self.get_current_token()?;
        match token.token_type {
            TokenType::EQUAL => Ok(true),
            TokenType::NEWLINE => Ok(false),
            _ => Err(CodeError::new(
                token.line,
                token.start,
                token.end,
                "Invalid Assignment",
            )),
        }
    }

    fn try_get_string_value(&mut self) -> Result<TextExpression, CodeError> {
        let mut exp_concat = Vec::new();

        while match self.get_current_token()?.token_type {
            TokenType::VALUE | TokenType::IDENTIFIER => {
                exp_concat.push(TextExpression::from_token(self.get_current_token()?)?);
                self.next_token();
                self.token_idx < self.tokens.len()
            }
            _ => false,
        } {}

        if exp_concat.is_empty() {
            let token = self.get_current_token()?;
            Err(CodeError::new(
                token.line,
                token.start,
                token.end,
                "Expected string value",
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

    fn try_get_number_value(&self) -> Result<NumberExpression, CodeError> {
        NumberExpression::from_token(self.get_current_token()?)
    }
}
