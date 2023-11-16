use crate::{
    error::CodeError,
    lexer::{Token, TokenType},
};

#[derive(Debug, PartialEq)]
pub struct IdentifierExpression {
    pub line: usize,
    pub start: usize,
    pub end: usize,
    pub var_name: String,
}

impl IdentifierExpression {
    pub fn new(line: usize, start: usize, end: usize, var_name: String) -> Self {
        IdentifierExpression {
            line,
            start,
            end,
            var_name,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum NumberExpressionTypes {
    Value(i64),
    Identifier(IdentifierExpression),
}

#[derive(Debug, PartialEq)]
pub enum TextExpressionTypes {
    Concat(TextExpression, TextExpression),
    Value(String),
    Identifier(IdentifierExpression),
}
impl TextExpressionTypes {
    pub fn is_identifier(&self) -> bool {
        match self {
            TextExpressionTypes::Identifier(_) => true,
            TextExpressionTypes::Concat(exp1, _) => exp1.get_expression().is_identifier(),
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ExpressionTypes {
    TextAssignment(IdentifierExpression, TextExpression),
    NumberAssignment(IdentifierExpression, NumberExpression),
    InputStatement(IdentifierExpression),
    OutputStatement(IdentifierExpression),
}

#[derive(Debug, PartialEq)]
pub struct Expression {
    pub line: usize,
    pub start: usize,
    pub end: usize,
    expression: ExpressionTypes,
}
impl Expression {
    pub fn new(line: usize, start: usize, end: usize, expression: ExpressionTypes) -> Self {
        Expression {
            line,
            expression,
            start,
            end,
        }
    }
    pub fn get_expression(&self) -> &ExpressionTypes {
        &self.expression
    }
}

#[derive(Debug, PartialEq)]
pub struct TextExpression {
    pub line: usize,
    pub start: usize,
    pub end: usize,
    expression: Box<TextExpressionTypes>,
}
impl TextExpression {
    pub fn new(line: usize, start: usize, end: usize, expression: TextExpressionTypes) -> Self {
        TextExpression {
            line,
            start,
            end,
            expression: Box::new(expression),
        }
    }
    pub fn from_token(token: &Token) -> Result<Self, CodeError> {
        match token.token_type {
            TokenType::VALUE => Ok(Self {
                line: token.line,
                start: token.start,
                end: token.end,
                expression: Box::new(TextExpressionTypes::Value(token.value.to_owned())),
            }),
            TokenType::IDENTIFIER => Ok(Self {
                line: token.line,
                start: token.start,
                end: token.end,
                expression: Box::new(TextExpressionTypes::Identifier(IdentifierExpression::new(
                    token.line,
                    token.start,
                    token.end,
                    token.value.to_owned(),
                ))),
            }),
            _ => Err(CodeError::new(
                token.line,
                token.start,
                token.end,
                "Error while parsing value!",
            )),
        }
    }
    pub fn join(text_exp1: TextExpression, text_exp2: TextExpression) -> Self {
        TextExpression::new(
            text_exp1.line,
            text_exp1.start,
            text_exp2.end,
            TextExpressionTypes::Concat(text_exp1, text_exp2),
        )
    }
    pub fn get_expression(&self) -> &TextExpressionTypes {
        self.expression.as_ref()
    }
}

#[derive(Debug, PartialEq)]
pub struct NumberExpression {
    pub line: usize,
    pub start: usize,
    pub end: usize,
    expression: Box<NumberExpressionTypes>,
}

impl NumberExpression {
    pub fn new(line: usize, start: usize, end: usize, expression: NumberExpressionTypes) -> Self {
        NumberExpression {
            line,
            start,
            end,
            expression: Box::new(expression),
        }
    }
    pub fn from_token(token: &Token) -> Result<Self, CodeError> {
        match token.token_type {
            TokenType::VALUE => {
                if let Ok(i64_val) = token.value.parse::<i64>() {
                    Ok(Self {
                        line: token.line,
                        start: token.start,
                        end: token.end,
                        expression: Box::new(NumberExpressionTypes::Value(i64_val)),
                    })
                } else {
                    Err(CodeError::new(
                        token.line,
                        token.start,
                        token.end,
                        "Could not parse number",
                    ))
                }
            }
            TokenType::IDENTIFIER => Ok(Self {
                line: token.line,
                start: token.start,
                end: token.end,
                expression: Box::new(NumberExpressionTypes::Identifier(
                    IdentifierExpression::new(
                        token.line,
                        token.start,
                        token.end,
                        token.value.to_owned(),
                    ),
                )),
            }),
            _ => Err(CodeError::new(
                token.line,
                token.start,
                token.end,
                "Error while parsing value!",
            )),
        }
    }
    pub fn get_expression(&self) -> &NumberExpressionTypes {
        self.expression.as_ref()
    }
}
