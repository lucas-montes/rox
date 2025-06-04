use crate::{
    environment::Environment,
    syntax_tree::{BinaryOperator, Expr, Literal, LogicalOperator, Stmt, UnaryOperator},
};

#[derive(Debug)]
pub enum InterpreterError {
    UndefinedVariable,
    WrongValue,
}

pub type InterpreterResult = Result<Literal, InterpreterError>;

#[derive(Default, Debug)]
pub struct Interpreter {
    env: Environment,
}

impl<'a> Interpreter {
    pub fn evaluate(&mut self, stmt: &'a Stmt<'a>) -> Result<(), InterpreterError> {
        self.evaluate_statement(stmt)
    }

    fn evaluate_statement(&mut self, stmt: &'a Stmt<'a>) -> Result<(), InterpreterError> {
        match stmt {
            Stmt::Expression(expr) => {
                self.evaluate_expression(expr)?;
            }
            Stmt::Print(expr) => {
                let result = self.evaluate_expression(expr)?;
                println!("{}", result);
            }
            Stmt::Block(stmts) => self.evaluate_block(stmts)?,
            Stmt::If(cond, then_stmt, else_branch) => {
                if self.evaluate_expression(cond)?.is_truthy() {
                    self.evaluate_statement(then_stmt)?;
                } else if let Some(else_stmt) = else_branch {
                    self.evaluate_statement(else_stmt)?;
                }
            }
            Stmt::Var(var, expr) => {
                let result = expr
                    .as_ref()
                    .map(|t| self.evaluate_expression(t))
                    .transpose()?
                    .unwrap_or(Literal::Nil);
                self.env.define(var, result);
            }
        };
        Ok(())
    }

    fn evaluate_block(&mut self, stmts: &[Stmt<'a>]) -> Result<(), InterpreterError> {
        self.env.push_scope();

        let result = (|| {
            for stmt in stmts {
                self.evaluate_statement(stmt)?;
            }
            Ok(())
        })();

        self.env.pop_scope();

        result
    }

    fn evaluate_expression(&mut self, expr: &'a Expr<'a>) -> InterpreterResult {
        match expr {
            Expr::Literal(lit) => Ok(lit.to_owned()),
            Expr::Grouping(expr) => self.evaluate_expression(expr),
            Expr::Unary(op, expr) => self.evaluate_unary(op, expr),
            Expr::Binary(exprl, op, exprr) => self.evaluate_binary(op, exprl, exprr),
            Expr::Logical(exprl, op, exprr) => self.evaluate_logical(op, exprl, exprr),
            Expr::Assign(token, expr) => {
                let val = self.evaluate_expression(expr)?;
                self.env
                    .assing(token.value(), val)
                    .ok_or(InterpreterError::UndefinedVariable)
            }
            Expr::Variable(token) => self
                .env
                .get(token.value())
                .map(|t| t.to_owned())
                .ok_or(InterpreterError::UndefinedVariable),
        }
    }

    fn evaluate_logical(
        &mut self,
        op: &'a LogicalOperator,
        exprl: &'a Expr<'a>,
        exprr: &'a Expr<'a>,
    ) -> InterpreterResult {
        let litl = self.evaluate_expression(exprl)?;
        let is_truthy = litl.is_truthy();
        if op.eq(&LogicalOperator::Or) {
            if is_truthy {
                return Ok(litl);
            }
        } else if !is_truthy {
            return Ok(litl);
        }
        self.evaluate_expression(exprr)
    }
    fn evaluate_binary(
        &mut self,
        op: &'a BinaryOperator,
        exprl: &'a Expr<'a>,
        exprr: &'a Expr<'a>,
    ) -> InterpreterResult {
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
    fn evaluate_unary(&mut self, op: &'a UnaryOperator, expr: &'a Expr<'a>) -> InterpreterResult {
        let lit = self.evaluate_expression(expr)?;
        match op {
            UnaryOperator::Minus => match lit {
                Literal::Number(v) => Ok(Literal::Number(-v)),
                _ => todo!(),
            },
            //TODO: use the is truthy or leverage this idea
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
                Literal::Nil => Ok(Literal::True),
                Literal::String(..) => Ok(Literal::False),
            },
        }
    }
}
