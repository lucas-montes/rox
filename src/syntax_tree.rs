use std::borrow::Cow;

use crate::tokens::{Token, TokenType};

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperator {
    Minus,
    Bang,
}

impl From<&TokenType> for UnaryOperator {
    fn from(value: &TokenType) -> Self {
        match value {
            TokenType::Minus => Self::Minus,
            TokenType::Bang => Self::Bang,
            _ => todo!(),
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub enum Literal<'a> {
    String(&'a str),
    Number(f64),
    False,
    True,
    Nil,
}
impl<'a> From<Token<'a>> for Literal<'a> {
    fn from(value: Token<'a>) -> Self {
        match value.kind() {
            TokenType::False => Self::False,
            TokenType::True => Self::True,
            TokenType::Nil => Self::Nil,
            TokenType::String => Self::String(value.value()),
            TokenType::Number => Self::Number(value.value().parse().unwrap()),
            _ => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperator {
    Slash,
    Star,
    Plus,
    Minus,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    BangEqual,
    EqualEqual,
}
impl From<&TokenType> for BinaryOperator {
    fn from(value: &TokenType) -> Self {
        match value {
            TokenType::Star => Self::Star,
            TokenType::EqualEqual => Self::EqualEqual,
            TokenType::BangEqual => Self::BangEqual,
            TokenType::Slash => Self::Slash,
            TokenType::Plus => Self::Plus,
            TokenType::Minus => Self::Minus,
            TokenType::Greater => Self::Greater,
            TokenType::GreaterEqual => Self::GreaterEqual,
            TokenType::Less => Self::Less,
            TokenType::LessEqual => Self::LessEqual,
            _ => todo!(),
        }
    }
}


#[derive(Debug, PartialEq, Clone)]
pub enum Expr<'a> {
    Literal(Literal<'a>),
    Grouping(Box<Expr<'a>>),
    Unary(UnaryOperator, Box<Expr<'a>>),
    Binary(Box<Expr<'a>>, BinaryOperator, Box<Expr<'a>>),
}

impl<'a> Expr<'a> {
    pub fn binary(expr: Expr<'a>, op: BinaryOperator, right: Expr<'a>) -> Self {
        Self::Binary(Box::new(expr), op, Box::new(right))
    }

    pub fn unary(op: UnaryOperator, expr: Expr<'a>) -> Self {
        Self::Unary(op, Box::new(expr))
    }

    pub fn grouping(expr: Expr<'a>) -> Self {
        Self::Grouping(Box::new(expr))
    }
    pub fn literal(expr: Literal<'a>) -> Self {
        Self::Literal(expr)
    }
}
