use std::collections::HashMap;

use crate::{
    error::CodeError,
    parser::{
        Expression, ExpressionTypes, IdentifierExpression, NumberExpression, NumberExpressionTypes,
        TextExpression, TextExpressionTypes,
    },
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

    pub fn run(&mut self, parsed_expressions: Vec<Expression>) -> Result<(), CodeError> {
        for expression in parsed_expressions {
            match expression.get_expression() {
                ExpressionTypes::TextAssignment(var_expr, value) => {
                    self.assign_string(&expression, var_expr, value)?;
                }
                ExpressionTypes::NumberAssignment(var_expr, value) => {
                    self.assign_number(&expression, var_expr, value)?;
                }
                ExpressionTypes::InputStatement(_) => todo!(),
                ExpressionTypes::OutputStatement(var_expr) => {
                    self.output(var_expr)?;
                }
            }
        }
        Ok(())
    }

    pub fn output(&mut self, var_expr: &IdentifierExpression) -> Result<(), CodeError> {
        let val = self.get_var(&var_expr, &var_expr.var_name)?;

        if let Some(val) = val.as_string() {
            println!("{}", val);
            Ok(())
        } else {
            Err(CodeError::new(
                var_expr.line,
                var_expr.start,
                var_expr.end,
                format!("Could not print variable {} as text!", var_expr.var_name).as_str(),
            ))
        }
    }

    pub fn set_var(&mut self, var_name: &String, value: RuntimeTypes) {
        self.variables.insert(var_name.to_owned(), value);
    }

    pub fn get_var(
        &mut self,
        expr: &IdentifierExpression,
        var_name: &String,
    ) -> Result<RuntimeTypes, CodeError> {
        if let Some(value) = self.variables.get(var_name) {
            Ok(value.clone())
        } else {
            Err(CodeError::new(
                expr.line,
                expr.start,
                expr.end,
                format!("Variable {} not set!", var_name).as_str(),
            ))
        }
    }

    pub fn eval_number_expression(
        &mut self,
        expr: &Expression,
        n_expr: &NumberExpression,
    ) -> Result<i64, CodeError> {
        match n_expr.get_expression() {
            NumberExpressionTypes::Value(value) => Ok(*value),
            NumberExpressionTypes::Identifier(var_expr) => {
                if let Some(value) = self.get_var(var_expr, &var_expr.var_name)?.as_number() {
                    Ok(value)
                } else {
                    Err(CodeError::new(
                        expr.line,
                        var_expr.start,
                        var_expr.end,
                        format!(
                            "Could not convert Text variable {} to number!",
                            var_expr.var_name
                        )
                        .as_str(),
                    ))
                }
            }
        }
    }

    pub fn eval_string_expression(
        &mut self,
        expr: &Expression,
        t_expr: &TextExpression,
    ) -> Result<String, CodeError> {
        match t_expr.get_expression() {
            TextExpressionTypes::Concat(expr1, expr2) => {
                if expr1.get_expression().is_identifier() || expr2.get_expression().is_identifier()
                {
                    Ok(format!(
                        "{}{}",
                        self.eval_string_expression(expr, expr1)?,
                        self.eval_string_expression(expr, expr2)?
                    ))
                } else {
                    Ok(format!(
                        "{} {}",
                        self.eval_string_expression(expr, expr1)?,
                        self.eval_string_expression(expr, expr2)?
                    ))
                }
            }
            TextExpressionTypes::Value(value) => Ok(value.to_owned()),
            TextExpressionTypes::Identifier(var_expr) => {
                if let Some(string) = self.get_var(var_expr, &var_expr.var_name)?.as_string() {
                    Ok(string)
                } else {
                    Err(CodeError::new(
                        t_expr.line,
                        t_expr.start,
                        t_expr.end,
                        format!(
                            "Could not convert variable {} to string!",
                            var_expr.var_name
                        )
                        .as_str(),
                    ))
                }
            }
        }
    }

    pub fn assign_number(
        &mut self,
        expr: &Expression,
        var_expr: &IdentifierExpression,
        n_expr: &NumberExpression,
    ) -> Result<(), CodeError> {
        let value = self.eval_number_expression(expr, n_expr)?;
        self.set_var(&var_expr.var_name, RuntimeTypes::Number(value));
        Ok(())
    }

    pub fn assign_string(
        &mut self,
        expr: &Expression,
        var_expr: &IdentifierExpression,
        s_expr: &TextExpression,
    ) -> Result<(), CodeError> {
        let value = self.eval_string_expression(expr, s_expr)?;
        self.set_var(&var_expr.var_name, RuntimeTypes::String(value));
        Ok(())
    }
}
