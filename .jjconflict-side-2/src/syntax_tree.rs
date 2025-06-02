use std::{fmt::Display, sync::Arc};

use crate::{
    tokens::{Token, TokenType},
};

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

//TODO: if we can remove clone we could use box
#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    String(Arc<str>),
    Number(f64),
    False,
    True,
    Nil,
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(v) => write!(f, "{}", v),
            Self::Number(v) => write!(f, "{}", v),
            Self::False => write!(f, "False"),
            Self::True => write!(f, "True"),
            Self::Nil => write!(f, "Nil"),
            _ => todo!(),
        }
    }
}

impl<'a> From<Token<'a>> for Literal {
    fn from(value: Token<'a>) -> Self {
        match value.kind() {
            TokenType::False => Self::False,
            TokenType::True => Self::True,
            TokenType::Nil => Self::Nil,
            TokenType::String => Self::String(value.value().into()),
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
    Assign(Token<'a>, Box<Expr<'a>>),
    Literal(Literal),
    Grouping(Box<Expr<'a>>),
    Unary(UnaryOperator, Box<Expr<'a>>),
    Binary(Box<Expr<'a>>, BinaryOperator, Box<Expr<'a>>),
    Variable(Token<'a>),
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

    pub fn assign(token:Token<'a> ,expr: Expr<'a>) -> Self {
        Self::Assign(token, Box::new(expr))
    }

    pub fn literal(expr: Literal) -> Self {
        Self::Literal(expr)
    }
}

#[derive(Debug)]
pub enum Stmt<'a> {
    Expression(Expr<'a>),
    Print(Expr<'a>),
    Var(&'a str, Option<Expr<'a>>),
}
