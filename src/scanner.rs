use std::fmt::Display;

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
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
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

impl TokenType {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '(' => Some(TokenType::LeftParen),
            ')' => Some(TokenType::RightParen),
            '{' => Some(TokenType::LeftBrace),
            '}' => Some(TokenType::RightBrace),
            ',' => Some(TokenType::Comma),
            '.' => Some(TokenType::Dot),
            '-' => Some(TokenType::Minus),
            '+' => Some(TokenType::Plus),
            ';' => Some(TokenType::Semicolon),
            '/' => Some(TokenType::Slash),
            '*' => Some(TokenType::Star),
            '!' => Some(TokenType::Bang),
            '=' => Some(TokenType::Equal),
            '>' => Some(TokenType::Greater),
            '<' => Some(TokenType::Less),
            _ => None,
        }
    }
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
    pub fn new(kind: TokenType, lexem: &'a str, line: u64) -> Self {
        Self { kind, lexem, line }
    }
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "type:{:?} lexem: {} line{}",
            self.kind, self.lexem, self.line
        )
    }
}

struct Scanner<'a> {
    source: String,
    tokens: Vec<Token<'a>>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
        }
    }
    pub fn scan(self) -> Self {
        for (line_num, line) in self.source.lines().enumerate() {
            handle_line(line_num, line);
        }
        self
    }
}

fn handle_line(line_num: usize, line: &str) {
    let mut chars = line.char_indices();
    let mut tokens = Vec::new();
    let line_num = line_num as u64 + 1;
    while let Some((char_pos, c)) = chars.next() {
        match TokenType::from_char(c) {
            Some(token) => {
                let v: *const u8 = line[char_pos..char_pos + 1].as_ptr();
                tokens.push(Token::new(token, v, line_num));
            }
            None => {}
        }
    }
}

struct ScannerIter {
    line: u64,
}

impl<'a> Iterator for ScannerIter {
    type Item = Token<'a>;
    fn next(&mut self) -> Option<Self::Item> {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_line() {
        let source = r#"fun greet(name) {
        print "Hello, " + name + "!";
        return nil;
    }"#
        .to_string();
        let mut v = source.lines().enumerate();
        let first = v.next().unwrap();
        let resutl = handle_line(first.0, first.1);
    }

    #[test]
    fn test_simple_function_declaration() {
        let source = r#"fun greet(name) {
        print "Hello, " + name + "!";
        return nil;
    }"#
        .to_string();
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

    #[test]
    fn test_class_with_methods() {
        let source = r#"class Person {
        init(name, age) {
            this.name = name;
            this.age = age;
        }

        sayHello() {
            print "Hi, I'm " + this.name;
        }
    }"#
        .to_string();
        let expected_tokens = vec![
            Token::new(TokenType::Class, "class", 1),
            Token::new(TokenType::Identifier, "Person", 1),
            Token::new(TokenType::LeftBrace, "{", 1),
            Token::new(TokenType::Identifier, "init", 2),
            Token::new(TokenType::LeftParen, "(", 2),
            Token::new(TokenType::Identifier, "name", 2),
            Token::new(TokenType::Comma, ",", 2),
            Token::new(TokenType::Identifier, "age", 2),
            Token::new(TokenType::RightParen, ")", 2),
            Token::new(TokenType::LeftBrace, "{", 2),
            Token::new(TokenType::This, "this", 3),
            Token::new(TokenType::Dot, ".", 3),
            Token::new(TokenType::Identifier, "name", 3),
            Token::new(TokenType::Equal, "=", 3),
            Token::new(TokenType::Identifier, "name", 3),
            Token::new(TokenType::Semicolon, ";", 3),
            Token::new(TokenType::This, "this", 4),
            Token::new(TokenType::Dot, ".", 4),
            Token::new(TokenType::Identifier, "age", 4),
            Token::new(TokenType::Equal, "=", 4),
            Token::new(TokenType::Identifier, "age", 4),
            Token::new(TokenType::Semicolon, ";", 4),
            Token::new(TokenType::RightBrace, "}", 5),
            Token::new(TokenType::Identifier, "sayHello", 7),
            Token::new(TokenType::LeftParen, "(", 7),
            Token::new(TokenType::RightParen, ")", 7),
            Token::new(TokenType::LeftBrace, "{", 7),
            Token::new(TokenType::Print, "print", 8),
            Token::new(TokenType::String, "\"Hi, I'm \"", 8),
            Token::new(TokenType::Plus, "+", 8),
            Token::new(TokenType::This, "this", 8),
            Token::new(TokenType::Dot, ".", 8),
            Token::new(TokenType::Identifier, "name", 8),
            Token::new(TokenType::Semicolon, ";", 8),
            Token::new(TokenType::RightBrace, "}", 9),
            Token::new(TokenType::RightBrace, "}", 10),
            Token::new(TokenType::Eof, "", 10),
        ];
        let scanner = Scanner::new(source);
        assert_eq!(scanner.scan().tokens, expected_tokens);
    }

    #[test]
    fn test_for_loop_with_counter() {
        let source = r#"for (var i = 0; i < 10; i = i + 1) {
        print i;
    }"#
        .to_string();
        let expected_tokens = vec![
            Token::new(TokenType::For, "for", 1),
            Token::new(TokenType::LeftParen, "(", 1),
            Token::new(TokenType::Var, "var", 1),
            Token::new(TokenType::Identifier, "i", 1),
            Token::new(TokenType::Equal, "=", 1),
            Token::new(TokenType::Number, "0", 1),
            Token::new(TokenType::Semicolon, ";", 1),
            Token::new(TokenType::Identifier, "i", 1),
            Token::new(TokenType::Less, "<", 1),
            Token::new(TokenType::Number, "10", 1),
            Token::new(TokenType::Semicolon, ";", 1),
            Token::new(TokenType::Identifier, "i", 1),
            Token::new(TokenType::Equal, "=", 1),
            Token::new(TokenType::Identifier, "i", 1),
            Token::new(TokenType::Plus, "+", 1),
            Token::new(TokenType::Number, "1", 1),
            Token::new(TokenType::RightParen, ")", 1),
            Token::new(TokenType::LeftBrace, "{", 1),
            Token::new(TokenType::Print, "print", 2),
            Token::new(TokenType::Identifier, "i", 2),
            Token::new(TokenType::Semicolon, ";", 2),
            Token::new(TokenType::RightBrace, "}", 3),
            Token::new(TokenType::Eof, "", 3),
        ];
        let scanner = Scanner::new(source);
        assert_eq!(scanner.scan().tokens, expected_tokens);
    }

    #[test]
    fn test_fibonacci_function() {
        let source = r#"fun fibonacci(n) {
        if (n <= 1) return n;
        return fibonacci(n - 1) + fibonacci(n - 2);
    }"#
        .to_string();
        let expected_tokens = vec![
            Token::new(TokenType::Fun, "fun", 1),
            Token::new(TokenType::Identifier, "fibonacci", 1),
            Token::new(TokenType::LeftParen, "(", 1),
            Token::new(TokenType::Identifier, "n", 1),
            Token::new(TokenType::RightParen, ")", 1),
            Token::new(TokenType::LeftBrace, "{", 1),
            Token::new(TokenType::If, "if", 2),
            Token::new(TokenType::LeftParen, "(", 2),
            Token::new(TokenType::Identifier, "n", 2),
            Token::new(TokenType::LessEqual, "<=", 2),
            Token::new(TokenType::Number, "1", 2),
            Token::new(TokenType::RightParen, ")", 2),
            Token::new(TokenType::Return, "return", 2),
            Token::new(TokenType::Identifier, "n", 2),
            Token::new(TokenType::Semicolon, ";", 2),
            Token::new(TokenType::Return, "return", 3),
            Token::new(TokenType::Identifier, "fibonacci", 3),
            Token::new(TokenType::LeftParen, "(", 3),
            Token::new(TokenType::Identifier, "n", 3),
            Token::new(TokenType::Minus, "-", 3),
            Token::new(TokenType::Number, "1", 3),
            Token::new(TokenType::RightParen, ")", 3),
            Token::new(TokenType::Plus, "+", 3),
            Token::new(TokenType::Identifier, "fibonacci", 3),
            Token::new(TokenType::LeftParen, "(", 3),
            Token::new(TokenType::Identifier, "n", 3),
            Token::new(TokenType::Minus, "-", 3),
            Token::new(TokenType::Number, "2", 3),
            Token::new(TokenType::RightParen, ")", 3),
            Token::new(TokenType::Semicolon, ";", 3),
            Token::new(TokenType::RightBrace, "}", 4),
            Token::new(TokenType::Eof, "", 4),
        ];
        let scanner = Scanner::new(source);
        assert_eq!(scanner.scan().tokens, expected_tokens);
    }

    #[test]
    fn test_variable_assignments_and_comparisons() {
        let source = r#"var x = 42;
    var y = 3.14;
    var isEqual = x == y;
    var isGreater = x > y;
    var name = "Alice";
    var isValid = name != nil and x >= 0;"#
            .to_string();
        let expected_tokens = vec![
            Token::new(TokenType::Var, "var", 1),
            Token::new(TokenType::Identifier, "x", 1),
            Token::new(TokenType::Equal, "=", 1),
            Token::new(TokenType::Number, "42", 1),
            Token::new(TokenType::Semicolon, ";", 1),
            Token::new(TokenType::Var, "var", 2),
            Token::new(TokenType::Identifier, "y", 2),
            Token::new(TokenType::Equal, "=", 2),
            Token::new(TokenType::Number, "3.14", 2),
            Token::new(TokenType::Semicolon, ";", 2),
            Token::new(TokenType::Var, "var", 3),
            Token::new(TokenType::Identifier, "isEqual", 3),
            Token::new(TokenType::Equal, "=", 3),
            Token::new(TokenType::Identifier, "x", 3),
            Token::new(TokenType::EqualEqual, "==", 3),
            Token::new(TokenType::Identifier, "y", 3),
            Token::new(TokenType::Semicolon, ";", 3),
            Token::new(TokenType::Var, "var", 4),
            Token::new(TokenType::Identifier, "isGreater", 4),
            Token::new(TokenType::Equal, "=", 4),
            Token::new(TokenType::Identifier, "x", 4),
            Token::new(TokenType::Greater, ">", 4),
            Token::new(TokenType::Identifier, "y", 4),
            Token::new(TokenType::Semicolon, ";", 4),
            Token::new(TokenType::Var, "var", 5),
            Token::new(TokenType::Identifier, "name", 5),
            Token::new(TokenType::Equal, "=", 5),
            Token::new(TokenType::String, "\"Alice\"", 5),
            Token::new(TokenType::Semicolon, ";", 5),
            Token::new(TokenType::Var, "var", 6),
            Token::new(TokenType::Identifier, "isValid", 6),
            Token::new(TokenType::Equal, "=", 6),
            Token::new(TokenType::Identifier, "name", 6),
            Token::new(TokenType::BangEqual, "!=", 6),
            Token::new(TokenType::Nil, "nil", 6),
            Token::new(TokenType::And, "and", 6),
            Token::new(TokenType::Identifier, "x", 6),
            Token::new(TokenType::GreaterEqual, ">=", 6),
            Token::new(TokenType::Number, "0", 6),
            Token::new(TokenType::Semicolon, ";", 6),
            Token::new(TokenType::Eof, "", 6),
        ];
        let scanner = Scanner::new(source);
        assert_eq!(scanner.scan().tokens, expected_tokens);
    }

    #[test]
    fn test_while_loop_with_conditions() {
        let source = r#"var count = 0;
    while (count < 5 and count >= 0) {
        print "Count: " + count;
        count = count + 1;
        if (count == 3) {
            print "Reached middle!";
        }
    }"#
        .to_string();
        let expected_tokens = vec![
            Token::new(TokenType::Var, "var", 1),
            Token::new(TokenType::Identifier, "count", 1),
            Token::new(TokenType::Equal, "=", 1),
            Token::new(TokenType::Number, "0", 1),
            Token::new(TokenType::Semicolon, ";", 1),
            Token::new(TokenType::While, "while", 2),
            Token::new(TokenType::LeftParen, "(", 2),
            Token::new(TokenType::Identifier, "count", 2),
            Token::new(TokenType::Less, "<", 2),
            Token::new(TokenType::Number, "5", 2),
            Token::new(TokenType::And, "and", 2),
            Token::new(TokenType::Identifier, "count", 2),
            Token::new(TokenType::GreaterEqual, ">=", 2),
            Token::new(TokenType::Number, "0", 2),
            Token::new(TokenType::RightParen, ")", 2),
            Token::new(TokenType::LeftBrace, "{", 2),
            Token::new(TokenType::Print, "print", 3),
            Token::new(TokenType::String, "\"Count: \"", 3),
            Token::new(TokenType::Plus, "+", 3),
            Token::new(TokenType::Identifier, "count", 3),
            Token::new(TokenType::Semicolon, ";", 3),
            Token::new(TokenType::Identifier, "count", 4),
            Token::new(TokenType::Equal, "=", 4),
            Token::new(TokenType::Identifier, "count", 4),
            Token::new(TokenType::Plus, "+", 4),
            Token::new(TokenType::Number, "1", 4),
            Token::new(TokenType::Semicolon, ";", 4),
            Token::new(TokenType::If, "if", 5),
            Token::new(TokenType::LeftParen, "(", 5),
            Token::new(TokenType::Identifier, "count", 5),
            Token::new(TokenType::EqualEqual, "==", 5),
            Token::new(TokenType::Number, "3", 5),
            Token::new(TokenType::RightParen, ")", 5),
            Token::new(TokenType::LeftBrace, "{", 5),
            Token::new(TokenType::Print, "print", 6),
            Token::new(TokenType::String, "\"Reached middle!\"", 6),
            Token::new(TokenType::Semicolon, ";", 6),
            Token::new(TokenType::RightBrace, "}", 7),
            Token::new(TokenType::RightBrace, "}", 8),
            Token::new(TokenType::Eof, "", 8),
        ];
        let scanner = Scanner::new(source);
        assert_eq!(scanner.scan().tokens, expected_tokens);
    }
}
