use std::ops::Not;

use crate::{
    syntax_tree::{Expr, Stmt},
    tokens::{Token, TokenType},
};

#[derive(Default, Debug)]
pub struct Parser<'a> {
    results: Vec<Stmt<'a>>,
    errors: Vec<ParserError>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        let mut results = Vec::with_capacity(tokens.len());
        let mut errors = Vec::new();

        for stmt in ParserIter::new(tokens) {
            match stmt {
                Ok(v) => results.push(v),
                Err(err) => errors.push(err),
            }
        }
        Self { results, errors }
    }

    pub fn results(self) -> Vec<Stmt<'a>> {
        self.results
    }

    pub fn errors(&self) -> Option<&[ParserError]> {
        self.errors.is_empty().not().then_some(&self.errors)
    }
}

#[derive(Debug)]
pub enum ParserError {
    MissingBrace,
    Missing,
    MissingSemicolon,
}

type ParserExprResult<'a> = Result<Expr<'a>, ParserError>;
type ParserResult<'a> = Result<Stmt<'a>, ParserError>;

struct ParserIter<'a> {
    inner: std::iter::Peekable<std::vec::IntoIter<Token<'a>>>,
}

impl<'a> ParserIter<'a> {
    fn new(tokens: Vec<Token<'a>>) -> Self {
        Self {
            inner: tokens.into_iter().peekable(),
        }
    }

    /// expression -> assignment ;
    fn expression(&mut self) -> ParserExprResult<'a> {
        self.assignment()
    }

    /// assignment -> IDENTIFIER "=" assignment | logic_or ;
    fn assignment(&mut self) -> ParserExprResult<'a> {
        let expr = self.logic_or()?;
        if self
            .inner
            .next_if(|t| matches!(t.kind(), TokenType::Equal))
            .is_some()
        {
            let value = self.assignment()?;

            match expr {
                Expr::Variable(token) => return Ok(Expr::assign(token, value)),
                _ => return Err(ParserError::Missing),
            }
        }
        Ok(expr)
    }

    /// primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" | IDENTIFIER;
    fn primary(&mut self) -> ParserExprResult<'a> {
        if let Some(token) = self.inner.next_if(|t| {
            matches!(
                t.kind(),
                TokenType::Nil
                    | TokenType::Number
                    | TokenType::String
                    | TokenType::False
                    | TokenType::True
                    | TokenType::LeftParen
                    | TokenType::Identifier
            )
        }) {
            return match token.kind() {
                TokenType::LeftParen => {
                    let expr = self.expression()?;
                    let _ = self
                        .inner
                        .next_if(|t| t.kind().eq(&TokenType::RightParen))
                        .ok_or(ParserError::Missing)?;

                    Ok(Expr::grouping(expr))
                }
                TokenType::Identifier => Ok(Expr::Variable(token)),
                _ => Ok(Expr::literal(token.into())),
            };
        }
        Err(ParserError::Missing)
    }

    /// unary -> ( "!" | "-" ) unary | primary;
    fn unary(&mut self) -> ParserExprResult<'a> {
        match self
            .inner
            .next_if(|t| matches!(t.kind(), TokenType::Bang | TokenType::Minus))
        {
            Some(token) => Ok(Expr::unary(token.kind().into(), self.unary()?)),
            None => self.primary(),
        }
    }

    /// factor -> unary ( ( "/" | "*" ) unary )* ;
    fn factor(&mut self) -> ParserExprResult<'a> {
        let mut expr = self.unary()?;
        while let Some(token) = self
            .inner
            .next_if(|t| matches!(t.kind(), TokenType::Slash | TokenType::Star))
        {
            expr = Expr::binary(expr, token.kind().into(), self.unary()?)
        }
        Ok(expr)
    }

    /// term -> factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> ParserExprResult<'a> {
        let mut expr = self.factor()?;
        while let Some(token) = self
            .inner
            .next_if(|t| matches!(t.kind(), TokenType::Minus | TokenType::Plus))
        {
            expr = Expr::binary(expr, token.kind().into(), self.factor()?)
        }
        Ok(expr)
    }

    /// comparaison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparaison(&mut self) -> ParserExprResult<'a> {
        let mut expr = self.term()?;
        while let Some(token) = self.inner.next_if(|t| {
            matches!(
                t.kind(),
                TokenType::Greater
                    | TokenType::GreaterEqual
                    | TokenType::Less
                    | TokenType::LessEqual
            )
        }) {
            expr = Expr::binary(expr, token.kind().into(), self.term()?)
        }
        Ok(expr)
    }

    /// equality -> comparaison ( ( "!=" | "==" ) comparaison )* ;
    fn equality(&mut self) -> ParserExprResult<'a> {
        let mut expr = self.comparaison()?;
        while let Some(token) = self
            .inner
            .next_if(|t| matches!(t.kind(), TokenType::BangEqual | TokenType::EqualEqual))
        {
            expr = Expr::binary(expr, token.kind().into(), self.comparaison()?)
        }
        Ok(expr)
    }

    fn synchronize(&mut self) {
        while let Some(token) = self.inner.next() {
            if matches!(
                token.kind(),
                TokenType::Eof
                    | TokenType::Semicolon
                    | TokenType::Class
                    | TokenType::For
                    | TokenType::Fun
                    | TokenType::If
                    | TokenType::Print
                    | TokenType::Return
                    | TokenType::Var
                    | TokenType::While
            ) {
                return;
            }
        }
    }

    /// logic_or -> logic_and ( "or" logic_and )* ;
    fn logic_or(&mut self) -> ParserExprResult<'a> {
        let mut expr = self.logic_and()?;

        while let Some(token) = self.inner.next_if(|t| matches!(t.kind(), TokenType::Or)) {
            expr = Expr::logical(expr, token.kind().into(), self.logic_and()?)
        }
        Ok(expr)
    }

    /// logic_and -> equality ( "and" equality )* ;
    fn logic_and(&mut self) -> ParserExprResult<'a> {
        let mut expr = self.equality()?;

        while let Some(token) = self.inner.next_if(|t| matches!(t.kind(), TokenType::Or)) {
            expr = Expr::logical(expr, token.kind().into(), self.equality()?)
        }
        Ok(expr)
    }

    // Statements

    /// block -> "{" declaration* "}" ;
    fn block(&mut self) -> ParserResult<'a> {
        let capacity = self.inner.size_hint();
        let mut block = Vec::with_capacity(capacity.1.unwrap_or(capacity.0));

        while self
            .inner
            .peek()
            .is_some_and(|t| !matches!(t.kind(), TokenType::RightBrace | TokenType::Eof))
        {
            block.push(self.declaration()?);
        }

        self.inner
            .next_if(|t| t.kind().eq(&TokenType::RightBrace))
            .ok_or(ParserError::MissingBrace)?;
        Ok(Stmt::Block(block))
    }

    fn declaration(&mut self) -> ParserResult<'a> {
        match self.inner.next_if(|t| t.kind().eq(&TokenType::Var)) {
            Some(_) => self.variable_declaration(),
            None => self.statement(),
        }
    }

    /// statement -> exprStmt | ifStmt | printStmt | block ;
    fn statement(&mut self) -> ParserResult<'a> {
        if self
            .inner
            .next_if(|t| t.kind().eq(&TokenType::If))
            .is_some()
        {
            return self.if_statement();
        }
        if self
            .inner
            .next_if(|t| t.kind().eq(&TokenType::Print))
            .is_some()
        {
            return self.print_statement();
        }
        if self
            .inner
            .next_if(|t| t.kind().eq(&TokenType::LeftBrace))
            .is_some()
        {
            return self.block();
        }
        self.expression_statement()
    }

    /// ifStmt -> "if" "(" expression ")" statement ("else" statement )? ;
    fn if_statement(&mut self) -> ParserResult<'a> {
        let _ = self
            .inner
            .next_if(|t| t.kind().eq(&TokenType::LeftParen))
            .ok_or(ParserError::Missing)?;
        let condition = self.expression()?;
        let _ = self
            .inner
            .next_if(|t| t.kind().eq(&TokenType::RightParen))
            .ok_or(ParserError::Missing)?;

        let then = self.statement()?;
        let else_branch = self
            .inner
            .next_if(|t| t.kind().eq(&TokenType::Else))
            .map(|_| self.statement())
            .transpose()?;

        Ok(Stmt::if_statement(condition, then, else_branch))
    }

    fn variable_declaration(&mut self) -> ParserResult<'a> {
        let token = self
            .inner
            .next_if(|t| t.kind().eq(&TokenType::Identifier))
            .ok_or(ParserError::Missing)?;
        let mut expr = None;
        if self
            .inner
            .next_if(|t| t.kind().eq(&TokenType::Equal))
            .is_some()
        {
            expr = Some(self.expression()?);
        };
        self.consume_semicolon()?;
        Ok(Stmt::Var(token.value(), expr))
    }

    fn print_statement(&mut self) -> ParserResult<'a> {
        let expr = self.expression()?;
        self.consume_semicolon()?;
        Ok(Stmt::Print(expr))
    }

    fn expression_statement(&mut self) -> ParserResult<'a> {
        let expr = self.expression()?;
        self.consume_semicolon()?;
        Ok(Stmt::Expression(expr))
    }

    fn consume_semicolon(&mut self) -> Result<(), ParserError> {
        self.inner
            .next_if(|t| t.kind().eq(&TokenType::Semicolon))
            .ok_or(ParserError::MissingSemicolon)?;
        Ok(())
    }
}

impl<'a> Iterator for ParserIter<'a> {
    type Item = ParserResult<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.peek() {
            Some(t) => {
                if t.kind().eq(&TokenType::Eof) {
                    self.inner.next()?;
                    return None;
                };
                let declaration = self.declaration();
                //TODO: check if it makes sense
                if declaration.is_err() {
                    self.synchronize();
                }
                Some(declaration)
            }
            None => None,
        }
    }
}
