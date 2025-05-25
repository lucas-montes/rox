use std::{
     fmt::Display, iter::{ Peekable}, str::{CharIndices, }
};

#[derive(Debug)]
enum ScanError {
    UnexpectedCharacter(u64),
    TokenMissing(u64)
}

impl Display for ScanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScanError::UnexpectedCharacter(line) => {
                write!(f, "Unexpected character encountered at line {}", line)
            },
            ScanError::TokenMissing(line) => {
                write!(f, "Token missing at line {}", line)
            },
        }
    }
}

type ScanResult<T> = Result<T, ScanError>;

#[derive(Debug, PartialEq)]
enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Bang,
    Equal,
    Greater,
    Less,

    BangEqual,
    EqualEqual,
    GreaterEqual,
    LessEqual,

    Identifier,
    Number,
    String,
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}

#[derive(Debug)]
struct Token<'a> {
    kind: TokenType,
    lexem: &'a str,
    line: u64,
}

impl<'a> PartialEq for Token<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.lexem == other.lexem && self.line == other.line
    }
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenType, lexem:  &'a str, line: u64) -> Self {
        Self { kind, lexem, line }
    }
    fn eof(line: u64) -> Self {
        Self {
            kind: TokenType::Eof,
            lexem: "",
            line,
        }
    }
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "type:{:?} lexem: {} line{}",
            self.kind, self.lexem, self.line
        )
    }
}

#[derive(Default)]
pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
    errors: Vec<ScanError>,
}

impl<'a> Scanner<'a> {
    pub fn new(source:  &'a str) -> Self {
        Self {
            source,
            ..Default::default()
        }
    }
    pub fn scan(mut self) -> Self {
        for token_result in ScanIter::new(self.source) {
            match token_result {
                Ok(token) => self.tokens.push(token),
                Err(error) => self.errors.push(error),
            }
        }
        self
    }
}



struct ScanIter<'a> {
    line: u64,
    current: usize,
    start: usize,
    source:  &'a str,
    inner: Peekable<CharIndices<'a>>,
    eof_returned: bool,
}

impl<'a> ScanIter<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            line: 1,
            current: 0,
            start: 0,
            source,
            inner: source.char_indices().peekable(),
            eof_returned: false,
        }
    }

}

impl<'a> Iterator for ScanIter<'a> {
    type Item = ScanResult<Token<'a>>;
    fn next(&mut self) -> Option<Self::Item> {
        let Some((current_pos, current_char)) = self.inner.next() else {
            if self.eof_returned {
                return None;
            }
            self.eof_returned = true;
            return Some(Ok(Token::eof(self.line)));
        };

        let next_char = self.inner.peek().map(|n|n.1);


        match TokenKinds::from_char(current_char, next_char){
            TokenKinds::SingleChar(token_type) => {
                return Some(Ok(Token::new(token_type, &self.source[current_pos..current_pos + 1], self.line)));
            },
            TokenKinds::DoubleChar(token_type) => {
                let Some((next_pos, _)) = self.inner.next() else {
                    return Some(Err(ScanError::TokenMissing(self.line)));
                };
                return Some(Ok(Token::new(token_type, &self.source[current_pos..next_pos + 1], self.line)));
            },
            TokenKinds::Comment => {
                while let Some((_, c)) = self.inner.next() {
                    if c == '\n' {
                        self.line += 1;
                        return self.next();
                    }
                }
            },
            TokenKinds::NewLine => {
                self.line += 1;
                return self.next();
            }
            TokenKinds::String =>{
                while let Some((next_pos, c)) = self.inner.next() {
                    if c == '\n' {
                        self.line += 1;
                    } else if c== '"' {
                        //NOTE: we remove the quotes from the string
                        let lexem = &self.source[current_pos + 1..next_pos];
                        return Some(Ok(Token::new(TokenType::String, lexem, self.line)));
                    }
                }
            },
        };


        Some(Err(ScanError::UnexpectedCharacter(self.line)))
    }
}

enum TokenKinds {
    SingleChar(TokenType),
    DoubleChar(TokenType),
    Comment,
    NewLine,
    String
}

impl TokenKinds {
    fn from_char(c: char, next_c: Option<char>)->Self{
        match (c, next_c){
            ('(', _) => Self::SingleChar(TokenType::LeftParen),
            (')', _) => Self::SingleChar(TokenType::RightParen),
            ('{', _) => Self::SingleChar(TokenType::LeftBrace),
            ('}', _) => Self::SingleChar(TokenType::RightBrace),
            (',', _) => Self::SingleChar(TokenType::Comma),
            ('.', _) => Self::SingleChar(TokenType::Dot),
            ('-', _) => Self::SingleChar(TokenType::Minus),
            ('+', _) => Self::SingleChar(TokenType::Plus),
            (';', _) => Self::SingleChar(TokenType::Semicolon),
            ('*', _) => Self::SingleChar(TokenType::Star),
            ('!', Some('=')) => Self::DoubleChar(TokenType::BangEqual),
            ('=', Some('=')) => Self::DoubleChar(TokenType::EqualEqual),
            ('>', Some('=')) => Self::DoubleChar(TokenType::GreaterEqual),
            ('<', Some('=')) => Self::DoubleChar(TokenType::LessEqual),
            ('!', _) => Self::SingleChar(TokenType::Bang),
            ('=', _) => Self::SingleChar(TokenType::Equal),
            ('>', _) => Self::SingleChar(TokenType::Greater),
            ('<', _) => Self::SingleChar(TokenType::Less),
            ('/', Some('/')) => Self::Comment,
            ('/', _) => Self::SingleChar(TokenType::Slash),
            ('\n', _) => Self::NewLine,
            ('"', _) => Self::String,
            _=> todo!("Handle more token types or errors: {} and {:?}", c, next_c),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_single_character_tokens() {
        let source = r#"(){}.,-+;*"#;
        let expected_tokens = vec![
            Token::new(TokenType::LeftParen, "(", 1),
            Token::new(TokenType::RightParen, ")", 1),
            Token::new(TokenType::LeftBrace, "{", 1),
            Token::new(TokenType::RightBrace, "}", 1),
            Token::new(TokenType::Dot, ".", 1),
            Token::new(TokenType::Comma, ",", 1),
            Token::new(TokenType::Minus, "-", 1),
            Token::new(TokenType::Plus, "+", 1),
            Token::new(TokenType::Semicolon, ";", 1),
            Token::new(TokenType::Star, "*", 1),
            Token::new(TokenType::Eof, "", 1),
        ];
        let scanner = Scanner::new(source);
        assert_eq!(scanner.scan().tokens, expected_tokens);
    }

    #[test]
    fn test_single_character_tokens_multiline() {
        let source = "(){}.,\n-+;*";
        let expected_tokens = vec![
            Token::new(TokenType::LeftParen, "(", 1),
            Token::new(TokenType::RightParen, ")", 1),
            Token::new(TokenType::LeftBrace, "{", 1),
            Token::new(TokenType::RightBrace, "}", 1),
            Token::new(TokenType::Dot, ".", 1),
            Token::new(TokenType::Comma, ",", 1),
            Token::new(TokenType::Minus, "-", 2),
            Token::new(TokenType::Plus, "+", 2),
            Token::new(TokenType::Semicolon, ";", 2),
            Token::new(TokenType::Star, "*", 2),
            Token::new(TokenType::Eof, "", 2),
        ];
        let scanner = Scanner::new(source);
        assert_eq!(scanner.scan().tokens, expected_tokens);
    }

    #[test]
    fn test_simple_characters_tokens_with_comment() {
        let source = r#"()//!=.==>=*"#;
        let expected_tokens = vec![
            Token::new(TokenType::LeftParen, "(", 1),
            Token::new(TokenType::RightParen, ")", 1),
            Token::new(TokenType::Eof, "", 1),
        ];
        let scanner = Scanner::new(source);
        assert_eq!(scanner.scan().tokens, expected_tokens);
    }

    #[test]
    fn test_simple_characters_tokens_with_comment_and_new_line() {
        let source = "()//!=.==>\n=*";
        let expected_tokens = vec![
            Token::new(TokenType::LeftParen, "(", 1),
            Token::new(TokenType::RightParen, ")", 1),
            Token::new(TokenType::Equal, "=", 2),
            Token::new(TokenType::Star, "*", 2),
            Token::new(TokenType::Eof, "", 2),
        ];
        let scanner = Scanner::new(source);
        assert_eq!(scanner.scan().tokens, expected_tokens);
    }

    #[test]
    fn test_double_character_tokens() {
        let source = r#"()!=.==>=/"#;
        let expected_tokens = vec![
            Token::new(TokenType::LeftParen, "(", 1),
            Token::new(TokenType::RightParen, ")", 1),
            Token::new(TokenType::BangEqual, "!=", 1),
            Token::new(TokenType::Dot, ".", 1),
            Token::new(TokenType::EqualEqual, "==", 1),
            Token::new(TokenType::GreaterEqual, ">=", 1),
            Token::new(TokenType::Slash, "/", 1),
            Token::new(TokenType::Eof, "", 1),
        ];
        let scanner = Scanner::new(source);
        assert_eq!(scanner.scan().tokens, expected_tokens);
    }

    #[test]
    fn test_string_tokens() {
        let source = "()\"hey, como\"";
        let expected_tokens = vec![
            Token::new(TokenType::LeftParen, "(", 1),
            Token::new(TokenType::RightParen, ")", 1),
            Token::new(TokenType::String, "hey, como", 1),
            Token::new(TokenType::Eof, "", 1),
        ];
        let scanner = Scanner::new(source);
        assert_eq!(scanner.scan().tokens, expected_tokens);
    }

    #[test]
    fn test_string_tokens_new_line() {
        let source = "()\"hey,\n como\"";
        let expected_tokens = vec![
            Token::new(TokenType::LeftParen, "(", 1),
            Token::new(TokenType::RightParen, ")", 1),
            Token::new(TokenType::String, "hey,\n como", 2),
            Token::new(TokenType::Eof, "", 2),
        ];
        let scanner = Scanner::new(source);
        assert_eq!(scanner.scan().tokens, expected_tokens);
    }

    #[test]
    fn test_simple_function_declaration() {
        let source = r#"fun greet(name) {
        print "Hello, " + name + "!";
        return nil;
    }"#;
        let expected_tokens = vec![
            Token::new(TokenType::Fun, "fun", 1),
            Token::new(TokenType::Identifier, "greet", 1),
            Token::new(TokenType::LeftParen, "(", 1),
            Token::new(TokenType::Identifier, "name", 1),
            Token::new(TokenType::RightParen, ")", 1),
            Token::new(TokenType::LeftBrace, "{", 1),
            Token::new(TokenType::Print, "print", 2),
            Token::new(TokenType::String, "\"Hello, \"", 2),
            Token::new(TokenType::Plus, "+", 2),
            Token::new(TokenType::Identifier, "name", 2),
            Token::new(TokenType::Plus, "+", 2),
            Token::new(TokenType::String, "\"!\"", 2),
            Token::new(TokenType::Semicolon, ";", 2),
            Token::new(TokenType::Return, "return", 3),
            Token::new(TokenType::Nil, "nil", 3),
            Token::new(TokenType::Semicolon, ";", 3),
            Token::new(TokenType::RightBrace, "}", 4),
            Token::new(TokenType::Eof, "", 4),
        ];
        let scanner = Scanner::new(source);
        assert_eq!(scanner.scan().tokens, expected_tokens);
    }

}
