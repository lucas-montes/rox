use crate::{
    environment::Environment,
    syntax_tree::{BinaryOperator, Expr, Literal, Stmt, UnaryOperator},
};

#[derive(Debug)]
pub enum InterpreterError {
    WrongValue,
}

pub type InterpreterResult<'a> = Result<Literal<'a>, InterpreterError>;

#[derive(Default)]
pub struct Interpreter<'a> {
    env: Environment<'a>,
}

impl<'a> Interpreter<'a> {
    pub fn evaluate_statement(&mut self, stmt: &'a Stmt<'a>) -> Result<(), InterpreterError> {
        match stmt {
            Stmt::Expression(expr) => {
                self.evaluate_expression(&expr)?;
            }
            Stmt::Print(expr) => {
                let result = self.evaluate_expression(&expr)?;
                println!("{}", result);
            }
            Stmt::Var(var, expr) => {
                let result = expr
                    .as_ref()
                    .map(|t| self.evaluate_expression(t))
                    .transpose()?
                    .unwrap_or(Literal::Nil);
                self.env.insert(var, result);
            }
            _ => todo!(),
        };
        Ok(())
    }

    fn evaluate_expression(&mut self, expr: &'a Expr<'a>) -> InterpreterResult<'a> {
        match expr {
            Expr::Literal(lit) => Ok(lit.to_owned()),
            Expr::Grouping(expr) => self.evaluate_expression(expr),
            Expr::Unary(op, expr) => self.evaluate_unary(op, expr),
            Expr::Binary(exprl, op, exprr) => self.evaluate_binary(op, exprl, exprr),
            Expr::Variable(token) => self
                .env
                .get(token.value())
                .map(|t| t.to_owned())
                .ok_or(InterpreterError::WrongValue),
        }
    }

    fn evaluate_binary(
        &mut self,
        op: &'a BinaryOperator,
        exprl: &'a Expr<'a>,
        exprr: &'a Expr<'a>,
    ) -> InterpreterResult<'a> {
        let litl = self.evaluate_expression(exprl)?;
        let litr = self.evaluate_expression(exprr)?;
        match op {
            BinaryOperator::Less => match (litl, litr) {
                (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Number(l - r)),
                _ => todo!(),
            },
            BinaryOperator::Plus => match (litl, litr) {
                (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Number(l + r)),
                _ => todo!(),
            },
            BinaryOperator::Slash => match (litl, litr) {
                (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Number(l / r)),
                _ => todo!(),
            },
            BinaryOperator::Star => match (litl, litr) {
                (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Number(l * r)),
                _ => todo!(),
            },
            _ => {
                todo!()
            }
        }
    }
    fn evaluate_unary(
        &mut self,
        op: &'a UnaryOperator,
        expr: &'a Expr<'a>,
    ) -> InterpreterResult<'a> {
        let lit = self.evaluate_expression(expr)?;
        match op {
            UnaryOperator::Minus => match lit {
                Literal::Number(v) => Ok(Literal::Number(-v)),
                _ => todo!(),
            },
            UnaryOperator::Bang => match lit {
                Literal::False => Ok(Literal::True),
                Literal::True => Ok(Literal::False),
                Literal::Number(v) => {
                    let result = match v {
                        0.0 => Literal::True,
                        _ => Literal::False,
                    };
                    Ok(result)
                }
                _ => todo!(),
            },
        }
    }
}
