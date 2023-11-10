use std::collections::HashMap;

use crate::parser::{
    Expression, ExpressionTypes, NumberExpression, NumberExpressionTypes, TextExpression,
    TextExpressionTypes,
};

use super::RuntimeTypes;

pub struct Interpreter {
    variables: HashMap<String, RuntimeTypes>,
}
impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
        }
    }

    pub fn run(&mut self, parsed_expressions: Vec<Expression>) -> Result<(), String> {
        for expression in parsed_expressions {
            match expression.get_expression() {
                ExpressionTypes::TextAssignment(var_name, value) => {
                    self.assign_string(expression.line, var_name, value)?;
                }
                ExpressionTypes::NumberAssignment(var_name, value) => {
                    self.assign_number(expression.line, var_name, value)?;
                }
                ExpressionTypes::InputStatement(_) => todo!(),
                ExpressionTypes::OutputStatement(var_name) => {
                    self.output(expression.line, var_name)?;
                }
            }
        }
        Ok(())
    }

    pub fn output(&self, line: usize, var_name: &String) -> Result<(), String> {
        if let Some(val) = self.variables.get(var_name) {
            if let Some(val) = val.as_string() {
                println!("{}", val);
                Ok(())
            } else {
                Err(format!(
                    "Could not print variable {} on line {}",
                    var_name, line
                ))
            }
        } else {
            Err(format!(
                "Could not find variable {} on line {}",
                var_name, line
            ))
        }
    }

    pub fn set_var(&mut self, var_name: &String, value: RuntimeTypes) {
        self.variables.insert(var_name.to_owned(), value);
    }

    pub fn get_var(&mut self, line: usize, var_name: &String) -> Result<RuntimeTypes, String> {
        if let Some(value) = self.variables.get(var_name) {
            Ok(value.clone())
        } else {
            Err(format!(
                "Variable {} not set. Trying to read on line {}",
                var_name, line
            ))
        }
    }

    pub fn eval_number_expression(
        &mut self,
        line: usize,
        expr: &NumberExpression,
    ) -> Result<i64, String> {
        match expr.get_expression() {
            NumberExpressionTypes::Value(value) => Ok(*value),
            NumberExpressionTypes::Identifier(var_name) => {
                if let Some(value) = self.get_var(line, &var_name)?.as_number() {
                    Ok(value)
                } else {
                    Err(format!(
                        "Could not convert Text variable {} to number on line {}",
                        var_name, line
                    ))
                }
            }
        }
    }

    pub fn eval_string_expression(
        &mut self,
        line: usize,
        expr: &TextExpression,
    ) -> Result<String, String> {
        match expr.get_expression() {
            TextExpressionTypes::Concat(expr1, expr2) => {
                if expr1.get_expression().is_identifier() || expr2.get_expression().is_identifier()
                {
                    Ok(format!(
                        "{}{}",
                        self.eval_string_expression(line, expr1)?,
                        self.eval_string_expression(line, expr2)?
                    ))
                } else {
                    Ok(format!(
                        "{} {}",
                        self.eval_string_expression(line, expr1)?,
                        self.eval_string_expression(line, expr2)?
                    ))
                }
            }
            TextExpressionTypes::Value(value) => Ok(value.to_owned()),
            TextExpressionTypes::Identifier(var_name) => {
                if let Some(string) = self.get_var(line, &var_name)?.as_string() {
                    Ok(string)
                } else {
                    Err(format!(
                        "Could not read variable {} on line {}",
                        var_name, line
                    ))
                }
            }
        }
    }

    pub fn assign_number(
        &mut self,
        line: usize,
        var_name: &String,
        n_expr: &NumberExpression,
    ) -> Result<(), String> {
        let value = self.eval_number_expression(line, n_expr)?;
        self.set_var(var_name, RuntimeTypes::Number(value));
        Ok(())
    }

    pub fn assign_string(
        &mut self,
        line: usize,
        var_name: &String,
        s_expr: &TextExpression,
    ) -> Result<(), String> {
        let value = self.eval_string_expression(line, s_expr)?;
        self.set_var(var_name, RuntimeTypes::String(value));
        Ok(())
    }
}
