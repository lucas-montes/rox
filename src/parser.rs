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

    pub fn results(&self) -> &[Stmt<'a>] {
        &self.results
    }
}

#[derive(Debug)]
enum ParserError {
    Missing,
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

    fn expression(&mut self) -> ParserExprResult<'a> {
        self.equality()
    }

    /// primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
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

    fn statement(&mut self) -> ParserResult<'a> {
        match self.inner.next_if(|t| t.kind().eq(&TokenType::Print)) {
            Some(_) => self.print_statement(),
            None => self.expression_statement(),
        }
    }

    fn print_statement(&mut self) -> ParserResult<'a> {
        let expr = self.expression()?;
        self.inner
            .next_if(|t| t.kind().eq(&TokenType::Semicolon))
            .ok_or(ParserError::Missing)?;
        Ok(Stmt::Print(expr))
    }
    fn expression_statement(&mut self) -> ParserResult<'a> {
        let expr = self.expression()?;
        self.inner
            .next_if(|t| t.kind().eq(&TokenType::Semicolon))
            .ok_or(ParserError::Missing)?;
        Ok(Stmt::Expression(expr))
    }
}

impl<'a> Iterator for ParserIter<'a> {
    type Item = ParserResult<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.peek() {
            Some(t) => {
                if t.kind().eq(&TokenType::Eof) {
                    println!("im eod {:?}", self.inner);
                    self.inner.next()?;
                    return None;
                };
                println!("parser is in: {:?}", t);
                Some(self.statement())
            }
            None => None,
        }
    }
}
