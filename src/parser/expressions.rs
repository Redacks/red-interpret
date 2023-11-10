use crate::lexer::{Token, TokenType};

#[derive(Debug, PartialEq)]
pub enum NumberExpressionTypes {
    Value(i64),
    Identifier(String),
}

#[derive(Debug, PartialEq)]
pub enum TextExpressionTypes {
    Concat(TextExpression, TextExpression),
    Value(String),
    Identifier(String),
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
    TextAssignment(String, TextExpression),
    NumberAssignment(String, NumberExpression),
    InputStatement(String),
    OutputStatement(String),
}

#[derive(Debug, PartialEq)]
pub struct Expression {
    pub line: usize,
    expression: ExpressionTypes,
}
impl Expression {
    pub fn new(line: usize, expression: ExpressionTypes) -> Self {
        Expression { line, expression }
    }
    pub fn get_expression(&self) -> &ExpressionTypes {
        &self.expression
    }
}

#[derive(Debug, PartialEq)]
pub struct TextExpression {
    pub line: usize,
    expression: Box<TextExpressionTypes>,
}
impl TextExpression {
    pub fn new(line: usize, expression: TextExpressionTypes) -> Self {
        TextExpression {
            line,
            expression: Box::new(expression),
        }
    }
    pub fn from_token(token: Token) -> Result<Self, String> {
        let line = token.line;
        match token.token_type {
            TokenType::VALUE(value) => Ok(Self {
                line: line,
                expression: Box::new(TextExpressionTypes::Value(value)),
            }),
            TokenType::IDENTIFIER(identifier) => Ok(Self {
                line: line,
                expression: Box::new(TextExpressionTypes::Identifier(identifier)),
            }),
            _ => Err(format!("Could not parse string on line {}", line)),
        }
    }
    pub fn join(text_exp1: TextExpression, text_exp2: TextExpression) -> Self {
        TextExpression::new(
            text_exp1.line,
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
    expression: Box<NumberExpressionTypes>,
}

impl NumberExpression {
    pub fn new(line: usize, expression: NumberExpressionTypes) -> Self {
        NumberExpression {
            line,
            expression: Box::new(expression),
        }
    }
    pub fn from_token(token: Token) -> Result<Self, String> {
        let line = token.line;
        match token.token_type {
            TokenType::VALUE(value) => {
                if let Ok(i64_val) = value.parse::<i64>() {
                    Ok(Self {
                        line: line,
                        expression: Box::new(NumberExpressionTypes::Value(i64_val)),
                    })
                } else {
                    Err(format!("Could not parse number on line {}", line))
                }
            }
            TokenType::IDENTIFIER(identifier) => Ok(Self {
                line: line,
                expression: Box::new(NumberExpressionTypes::Identifier(identifier)),
            }),
            _ => Err(format!("Error while parsing line {}", line)),
        }
    }
    pub fn get_expression(&self) -> &NumberExpressionTypes {
        self.expression.as_ref()
    }
}
