use crate::syntax_tree::{BinaryOperator, Expr, Literal, Stmt, UnaryOperator};

#[derive(Debug)]
pub enum InterpreterError {
    WrongValue,
}

pub type InterpreterResult<'a> = Result<Literal<'a>, InterpreterError>;

pub fn evaluate_statement<'a>(stmt: &Stmt<'a>) -> Result<(), InterpreterError> {
    match stmt {
        Stmt::Expression(expr) => {evaluate_expression(&expr)?;},
        Stmt::Print(expr) => {
            let result = evaluate_expression(&expr);
            println!("{:?}", result);
        }
    };
    Ok(())
}

fn evaluate_expression<'a>(expr: &'a Expr<'a>) -> InterpreterResult<'a> {
    match expr {
        Expr::Literal(lit) => Ok(lit.to_owned()),
        Expr::Grouping(expr) => evaluate_expression(expr),
        Expr::Unary(op, expr) => evaluate_unary(op, expr),
        Expr::Binary(exprl, op, exprr) => evaluate_binary(op, exprl, exprr),
    }
}

fn evaluate_binary<'a>(
    op: &'a BinaryOperator,
    exprl: &'a Expr<'a>,
    exprr: &'a Expr<'a>,
) -> InterpreterResult<'a> {
    let litl = evaluate_expression(exprl)?;
    let litr = evaluate_expression(exprr)?;
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
fn evaluate_unary<'a>(op: &'a UnaryOperator, expr: &'a Expr<'a>) -> InterpreterResult<'a> {
    let lit = evaluate_expression(expr)?;
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
