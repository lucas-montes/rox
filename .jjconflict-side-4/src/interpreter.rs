use crate::{
    environment::Environment,
    syntax_tree::{BinaryOperator, Expr, Literal, LogicalOperator, Stmt, UnaryOperator},
};

#[derive(Debug)]
pub enum InterpreterError {
    UndefinedVariable,
    WrongValue,
    DivisionByZero,
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
            Stmt::While(cond, body) => {
                while self.evaluate_expression(cond)?.is_truthy() {
                    self.evaluate_statement(body)?
                }
            }
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
                (Literal::Number(l), Literal::Number(r)) => Ok(Literal::from_bool(l < r)),
                _ => Err(InterpreterError::WrongValue),
            },
            BinaryOperator::LessEqual => match (litl, litr) {
                (Literal::Number(l), Literal::Number(r)) => Ok(Literal::from_bool(l <= r)),
                _ => Err(InterpreterError::WrongValue),
            },
            BinaryOperator::Greater => match (litl, litr) {
                (Literal::Number(l), Literal::Number(r)) => Ok(Literal::from_bool(l > r)),
                _ => Err(InterpreterError::WrongValue),
            },
            BinaryOperator::GreaterEqual => match (litl, litr) {
                (Literal::Number(l), Literal::Number(r)) => Ok(Literal::from_bool(l >= r)),
                _ => Err(InterpreterError::WrongValue),
            },
            BinaryOperator::EqualEqual => Ok(Literal::from_bool(Self::is_equal(&litl, &litr))),
            BinaryOperator::BangEqual => Ok(Literal::from_bool(!Self::is_equal(&litl, &litr))),
            BinaryOperator::Plus => match (litl, litr) {
                (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Number(l + r)),
                (Literal::String(l), Literal::String(r)) => {
                    let concatenated = format!("{}{}", l, r);
                    Ok(Literal::String(concatenated.into()))
                }
                _ => Err(InterpreterError::WrongValue),
            },
            BinaryOperator::Minus => match (litl, litr) {
                (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Number(l - r)),
                _ => Err(InterpreterError::WrongValue),
            },
            BinaryOperator::Slash => match (litl, litr) {
                (Literal::Number(l), Literal::Number(r)) => {
                    if r == 0.0 {
                        Err(InterpreterError::DivisionByZero)
                    } else {
                        Ok(Literal::Number(l / r))
                    }
                }
                _ => Err(InterpreterError::WrongValue),
            },
            BinaryOperator::Star => match (litl, litr) {
                (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Number(l * r)),
                _ => Err(InterpreterError::WrongValue),
            },
        }
    }

    /// Helper method for equality comparison
    fn is_equal(left: &Literal, right: &Literal) -> bool {
        match (left, right) {
            (Literal::Number(l), Literal::Number(r)) => l == r,
            (Literal::String(l), Literal::String(r)) => l == r,
            (Literal::True, Literal::True)
            | (Literal::False, Literal::False)
            | (Literal::Nil, Literal::Nil) => true,
            _ => false,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        syntax_tree::{BinaryOperator, Expr, Literal, LogicalOperator, Stmt, UnaryOperator},
        tokens::{Token, TokenType},
    };

    fn create_token(value: &str, token_type: TokenType) -> Token {
        Token::new(token_type, value, 1)
    }

    #[test]
    fn test_literal_evaluation() {
        let mut interpreter = Interpreter::default();

        // Test number literal
        let expr = Expr::Literal(Literal::Number(42.0));
        let result = interpreter.evaluate_expression(&expr).unwrap();
        assert_eq!(result, Literal::Number(42.0));

        // Test string literal
        let expr = Expr::Literal(Literal::String("hello".into()));
        let result = interpreter.evaluate_expression(&expr).unwrap();
        assert_eq!(result, Literal::String("hello".into()));

        // Test boolean literals
        let expr = Expr::Literal(Literal::True);
        let result = interpreter.evaluate_expression(&expr).unwrap();
        assert_eq!(result, Literal::True);

        let expr = Expr::Literal(Literal::False);
        let result = interpreter.evaluate_expression(&expr).unwrap();
        assert_eq!(result, Literal::False);

        // Test nil literal
        let expr = Expr::Literal(Literal::Nil);
        let result = interpreter.evaluate_expression(&expr).unwrap();
        assert_eq!(result, Literal::Nil);
    }

    #[test]
    fn test_arithmetic_operations() {
        let mut interpreter = Interpreter::default();

        // Test addition
        let expr = Expr::Binary(
            Box::new(Expr::Literal(Literal::Number(5.0))),
            BinaryOperator::Plus,
            Box::new(Expr::Literal(Literal::Number(3.0))),
        );
        let result = interpreter.evaluate_expression(&expr).unwrap();
        assert_eq!(result, Literal::Number(8.0));

        // Test subtraction
        let expr = Expr::Binary(
            Box::new(Expr::Literal(Literal::Number(10.0))),
            BinaryOperator::Minus,
            Box::new(Expr::Literal(Literal::Number(4.0))),
        );
        let result = interpreter.evaluate_expression(&expr).unwrap();
        assert_eq!(result, Literal::Number(6.0));

        // Test multiplication
        let expr = Expr::Binary(
            Box::new(Expr::Literal(Literal::Number(6.0))),
            BinaryOperator::Star,
            Box::new(Expr::Literal(Literal::Number(7.0))),
        );
        let result = interpreter.evaluate_expression(&expr).unwrap();
        assert_eq!(result, Literal::Number(42.0));

        // Test division
        let expr = Expr::Binary(
            Box::new(Expr::Literal(Literal::Number(15.0))),
            BinaryOperator::Slash,
            Box::new(Expr::Literal(Literal::Number(3.0))),
        );
        let result = interpreter.evaluate_expression(&expr).unwrap();
        assert_eq!(result, Literal::Number(5.0));
    }

    #[test]
    fn test_division_by_zero() {
        let mut interpreter = Interpreter::default();

        let expr = Expr::Binary(
            Box::new(Expr::Literal(Literal::Number(10.0))),
            BinaryOperator::Slash,
            Box::new(Expr::Literal(Literal::Number(0.0))),
        );
        let result = interpreter.evaluate_expression(&expr);
        assert!(matches!(result, Err(InterpreterError::DivisionByZero)));
    }

    #[test]
    fn test_string_concatenation() {
        let mut interpreter = Interpreter::default();

        let expr = Expr::Binary(
            Box::new(Expr::Literal(Literal::String("Hello, ".into()))),
            BinaryOperator::Plus,
            Box::new(Expr::Literal(Literal::String("World!".into()))),
        );
        let result = interpreter.evaluate_expression(&expr).unwrap();
        assert_eq!(result, Literal::String("Hello, World!".into()));
    }

    #[test]
    fn test_comparison_operations() {
        let mut interpreter = Interpreter::default();

        // Test less than
        let expr = Expr::Binary(
            Box::new(Expr::Literal(Literal::Number(3.0))),
            BinaryOperator::Less,
            Box::new(Expr::Literal(Literal::Number(5.0))),
        );
        let result = interpreter.evaluate_expression(&expr).unwrap();
        assert_eq!(result, Literal::True);

        // Test greater than
        let expr = Expr::Binary(
            Box::new(Expr::Literal(Literal::Number(7.0))),
            BinaryOperator::Greater,
            Box::new(Expr::Literal(Literal::Number(3.0))),
        );
        let result = interpreter.evaluate_expression(&expr).unwrap();
        assert_eq!(result, Literal::True);

        // Test less than or equal
        let expr = Expr::Binary(
            Box::new(Expr::Literal(Literal::Number(5.0))),
            BinaryOperator::LessEqual,
            Box::new(Expr::Literal(Literal::Number(5.0))),
        );
        let result = interpreter.evaluate_expression(&expr).unwrap();
        assert_eq!(result, Literal::True);

        // Test greater than or equal
        let expr = Expr::Binary(
            Box::new(Expr::Literal(Literal::Number(8.0))),
            BinaryOperator::GreaterEqual,
            Box::new(Expr::Literal(Literal::Number(3.0))),
        );
        let result = interpreter.evaluate_expression(&expr).unwrap();
        assert_eq!(result, Literal::True);
    }

    #[test]
    fn test_equality_operations() {
        let mut interpreter = Interpreter::default();

        // Test number equality
        let expr = Expr::Binary(
            Box::new(Expr::Literal(Literal::Number(42.0))),
            BinaryOperator::EqualEqual,
            Box::new(Expr::Literal(Literal::Number(42.0))),
        );
        let result = interpreter.evaluate_expression(&expr).unwrap();
        assert_eq!(result, Literal::True);

        // Test string equality
        let expr = Expr::Binary(
            Box::new(Expr::Literal(Literal::String("test".into()))),
            BinaryOperator::EqualEqual,
            Box::new(Expr::Literal(Literal::String("test".into()))),
        );
        let result = interpreter.evaluate_expression(&expr).unwrap();
        assert_eq!(result, Literal::True);

        // Test inequality
        let expr = Expr::Binary(
            Box::new(Expr::Literal(Literal::Number(5.0))),
            BinaryOperator::BangEqual,
            Box::new(Expr::Literal(Literal::Number(3.0))),
        );
        let result = interpreter.evaluate_expression(&expr).unwrap();
        assert_eq!(result, Literal::True);

        // Test different types are not equal
        let expr = Expr::Binary(
            Box::new(Expr::Literal(Literal::Number(5.0))),
            BinaryOperator::EqualEqual,
            Box::new(Expr::Literal(Literal::String("5".into()))),
        );
        let result = interpreter.evaluate_expression(&expr).unwrap();
        assert_eq!(result, Literal::False);
    }

    #[test]
    fn test_unary_operations() {
        let mut interpreter = Interpreter::default();

        // Test unary minus
        let expr = Expr::Unary(
            UnaryOperator::Minus,
            Box::new(Expr::Literal(Literal::Number(5.0))),
        );
        let result = interpreter.evaluate_expression(&expr).unwrap();
        assert_eq!(result, Literal::Number(-5.0));

        // Test bang operator with true
        let expr = Expr::Unary(UnaryOperator::Bang, Box::new(Expr::Literal(Literal::True)));
        let result = interpreter.evaluate_expression(&expr).unwrap();
        assert_eq!(result, Literal::False);

        // Test bang operator with false
        let expr = Expr::Unary(UnaryOperator::Bang, Box::new(Expr::Literal(Literal::False)));
        let result = interpreter.evaluate_expression(&expr).unwrap();
        assert_eq!(result, Literal::True);

        // Test bang operator with nil
        let expr = Expr::Unary(UnaryOperator::Bang, Box::new(Expr::Literal(Literal::Nil)));
        let result = interpreter.evaluate_expression(&expr).unwrap();
        assert_eq!(result, Literal::True);

        // Test bang operator with number (zero)
        let expr = Expr::Unary(
            UnaryOperator::Bang,
            Box::new(Expr::Literal(Literal::Number(0.0))),
        );
        let result = interpreter.evaluate_expression(&expr).unwrap();
        assert_eq!(result, Literal::True);

        // Test bang operator with number (non-zero)
        let expr = Expr::Unary(
            UnaryOperator::Bang,
            Box::new(Expr::Literal(Literal::Number(42.0))),
        );
        let result = interpreter.evaluate_expression(&expr).unwrap();
        assert_eq!(result, Literal::False);
    }

    #[test]
    fn test_logical_operations() {
        let mut interpreter = Interpreter::default();

        // Test logical OR with true left operand
        let expr = Expr::Logical(
            Box::new(Expr::Literal(Literal::True)),
            LogicalOperator::Or,
            Box::new(Expr::Literal(Literal::False)),
        );
        let result = interpreter.evaluate_expression(&expr).unwrap();
        assert_eq!(result, Literal::True);

        // Test logical OR with false left operand
        let expr = Expr::Logical(
            Box::new(Expr::Literal(Literal::False)),
            LogicalOperator::Or,
            Box::new(Expr::Literal(Literal::True)),
        );
        let result = interpreter.evaluate_expression(&expr).unwrap();
        assert_eq!(result, Literal::True);

        // Test logical AND with true operands
        let expr = Expr::Logical(
            Box::new(Expr::Literal(Literal::True)),
            LogicalOperator::And,
            Box::new(Expr::Literal(Literal::True)),
        );
        let result = interpreter.evaluate_expression(&expr).unwrap();
        assert_eq!(result, Literal::True);

        // Test logical AND with false left operand
        let expr = Expr::Logical(
            Box::new(Expr::Literal(Literal::False)),
            LogicalOperator::And,
            Box::new(Expr::Literal(Literal::True)),
        );
        let result = interpreter.evaluate_expression(&expr).unwrap();
        assert_eq!(result, Literal::False);
    }

    #[test]
    fn test_grouping() {
        let mut interpreter = Interpreter::default();

        let expr = Expr::Grouping(Box::new(Expr::Binary(
            Box::new(Expr::Literal(Literal::Number(2.0))),
            BinaryOperator::Plus,
            Box::new(Expr::Literal(Literal::Number(3.0))),
        )));
        let result = interpreter.evaluate_expression(&expr).unwrap();
        assert_eq!(result, Literal::Number(5.0));
    }

    #[test]
    fn test_variable_declaration_and_access() {
        let mut interpreter = Interpreter::default();

        // Test variable declaration with value
        let stmt = Stmt::Var("x", Some(Expr::Literal(Literal::Number(42.0))));
        interpreter.evaluate(&stmt).unwrap();

        // Test variable access
        let token = create_token("x", TokenType::Identifier);
        let expr = Expr::Variable(token);
        let result = interpreter.evaluate_expression(&expr).unwrap();
        assert_eq!(result, Literal::Number(42.0));

        // Test variable declaration without value (should be nil)
        let stmt = Stmt::Var("y", None);
        interpreter.evaluate(&stmt).unwrap();

        let token = create_token("y", TokenType::Identifier);
        let expr = Expr::Variable(token);
        let result = interpreter.evaluate_expression(&expr).unwrap();
        assert_eq!(result, Literal::Nil);
    }

    #[test]
    fn test_undefined_variable() {
        let mut interpreter = Interpreter::default();

        let token = create_token("undefined", TokenType::Identifier);
        let expr = Expr::Variable(token);
        let result = interpreter.evaluate_expression(&expr);
        assert!(matches!(result, Err(InterpreterError::UndefinedVariable)));
    }

    #[test]
    fn test_variable_assignment() {
        let mut interpreter = Interpreter::default();

        // Declare variable
        let stmt = Stmt::Var("x", Some(Expr::Literal(Literal::Number(10.0))));
        interpreter.evaluate(&stmt).unwrap();

        // Assign new value
        let token = create_token("x", TokenType::Identifier);
        let expr = Expr::Assign(token, Box::new(Expr::Literal(Literal::Number(20.0))));
        let result = interpreter.evaluate_expression(&expr).unwrap();
        assert_eq!(result, Literal::Number(20.0));

        // Verify the variable was updated
        let token = create_token("x", TokenType::Identifier);
        let expr = Expr::Variable(token);
        let result = interpreter.evaluate_expression(&expr).unwrap();
        assert_eq!(result, Literal::Number(20.0));
    }

    #[test]
    fn test_print_statement() {
        let mut interpreter = Interpreter::default();

        let stmt = Stmt::Print(Expr::Literal(Literal::String("Hello, World!".into())));
        // This test mainly ensures no panic occurs
        interpreter.evaluate(&stmt).unwrap();
    }

    #[test]
    fn test_expression_statement() {
        let mut interpreter = Interpreter::default();

        let stmt = Stmt::Expression(Expr::Binary(
            Box::new(Expr::Literal(Literal::Number(5.0))),
            BinaryOperator::Plus,
            Box::new(Expr::Literal(Literal::Number(3.0))),
        ));
        interpreter.evaluate(&stmt).unwrap();
    }

    #[test]
    fn test_if_statement() {
        let mut interpreter = Interpreter::default();

        // Test if with true condition
        let stmt = Stmt::If(
            Expr::Literal(Literal::True),
            Box::new(Stmt::Var("x", Some(Expr::Literal(Literal::Number(42.0))))),
            None,
        );
        interpreter.evaluate(&stmt).unwrap();

        // Check that variable was declared
        let token = create_token("x", TokenType::Identifier);
        let expr = Expr::Variable(token);
        let result = interpreter.evaluate_expression(&expr).unwrap();
        assert_eq!(result, Literal::Number(42.0));
    }

    #[test]
    fn test_if_else_statement() {
        let mut interpreter = Interpreter::default();

        // Test if-else with false condition
        let stmt = Stmt::If(
            Expr::Literal(Literal::False),
            Box::new(Stmt::Var("x", Some(Expr::Literal(Literal::Number(10.0))))),
            Some(Box::new(Stmt::Var(
                "y",
                Some(Expr::Literal(Literal::Number(20.0))),
            ))),
        );
        interpreter.evaluate(&stmt).unwrap();

        // Check that else branch was executed
        let token = create_token("y", TokenType::Identifier);
        let expr = Expr::Variable(token);
        let result = interpreter.evaluate_expression(&expr).unwrap();
        assert_eq!(result, Literal::Number(20.0));

        // Check that if branch was not executed
        let token = create_token("x", TokenType::Identifier);
        let expr = Expr::Variable(token);
        let result = interpreter.evaluate_expression(&expr);
        assert!(matches!(result, Err(InterpreterError::UndefinedVariable)));
    }

    #[test]
    fn test_type_errors() {
        let mut interpreter = Interpreter::default();

        // Test arithmetic with wrong types
        let expr = Expr::Binary(
            Box::new(Expr::Literal(Literal::String("hello".into()))),
            BinaryOperator::Minus,
            Box::new(Expr::Literal(Literal::Number(5.0))),
        );
        let result = interpreter.evaluate_expression(&expr);
        assert!(matches!(result, Err(InterpreterError::WrongValue)));

        // Test comparison with wrong types
        let expr = Expr::Binary(
            Box::new(Expr::Literal(Literal::String("hello".into()))),
            BinaryOperator::Less,
            Box::new(Expr::Literal(Literal::Number(5.0))),
        );
        let result = interpreter.evaluate_expression(&expr);
        assert!(matches!(result, Err(InterpreterError::WrongValue)));
    }

    #[test]
    fn test_complex_expression() {
        let mut interpreter = Interpreter::default();

        // Test: (5 + 3) * 2 - 1
        let expr = Expr::Binary(
            Box::new(Expr::Binary(
                Box::new(Expr::Grouping(Box::new(Expr::Binary(
                    Box::new(Expr::Literal(Literal::Number(5.0))),
                    BinaryOperator::Plus,
                    Box::new(Expr::Literal(Literal::Number(3.0))),
                )))),
                BinaryOperator::Star,
                Box::new(Expr::Literal(Literal::Number(2.0))),
            )),
            BinaryOperator::Minus,
            Box::new(Expr::Literal(Literal::Number(1.0))),
        );
        let result = interpreter.evaluate_expression(&expr).unwrap();
        assert_eq!(result, Literal::Number(15.0)); // (5+3)*2-1 = 8*2-1 = 16-1 = 15
    }
}
