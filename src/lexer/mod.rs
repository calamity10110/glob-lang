// Lexer module - tokenizes source code

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Imp,
    Def,
    Fn,
    Asy,
    Cs,
    Mn,
    Own,
    Ref,
    Copy,
    Await,
    Loop,
    If,
    Elif,
    Else,
    For,
    While,
    In,
    Return,
    Break,
    Continue,
    Try,
    Catch,

    // Literals
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),

    // Identifiers
    Identifier(String),

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    Equal,
    EqualEqual,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
    Not,

    // Delimiters
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Colon,
    Dot,
    Arrow,

    // UI Syntax
    UiSprite(String),

    // Scientific Units
    Unit(String), // e.g., "m/s", "m/s^2", "kg"

    // Special
    Newline,
    Indent,
    Dedent,
    Eof,

    // Additional operators
    BangEqual,      // !=
    LessLess,       // <<
    GreaterGreater, // >>
    Ampersand,      // &
    Pipe,           // |
    Semicolon,      // ;
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Identifier(s) => write!(f, "Identifier({})", s),
            Token::Integer(n) => write!(f, "Integer({})", n),
            Token::Float(n) => write!(f, "Float({})", n),
            Token::String(s) => write!(f, "String(\"{}\")", s),
            Token::UiSprite(s) => write!(f, "UiSprite({})", s),
            _ => write!(f, "{:?}", self),
        }
    }
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let current_char = chars.get(0).copied();

        Lexer {
            input: chars,
            position: 0,
            current_char,
        }
    }

    fn advance(&mut self) {
        self.position += 1;
        self.current_char = self.input.get(self.position).copied();
    }

    fn peek(&self, offset: usize) -> Option<char> {
        self.input.get(self.position + offset).copied()
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.current_char {
            match ch {
                ' ' | '\t' => self.advance(),
                '\n' => {
                    tokens.push(Token::Newline);
                    self.advance();
                }
                '#' => {
                    if self.peek(1) == Some('[') {
                        self.skip_multiline_comment();
                    } else {
                        self.skip_comment();
                    }
                }
                '"' => tokens.push(self.read_string()),
                '0'..='9' => tokens.push(self.read_number()),
                'a'..='z' | 'A'..='Z' | '_' => tokens.push(self.read_identifier_or_unit()),
                '^' => {
                    // Check for UI sprite syntax: ^÷^[...]
                    if self.peek(1) == Some('÷')
                        && self.peek(2) == Some('^')
                        && self.peek(3) == Some('[')
                    {
                        tokens.push(self.read_ui_sprite());
                    } else {
                        tokens.push(Token::Caret);
                        self.advance();
                    }
                }
                '+' => {
                    tokens.push(Token::Plus);
                    self.advance();
                }
                '-' => {
                    if self.peek(1) == Some('>') {
                        tokens.push(Token::Arrow);
                        self.advance();
                        self.advance();
                    } else {
                        tokens.push(Token::Minus);
                        self.advance();
                    }
                }
                '*' => {
                    tokens.push(Token::Star);
                    self.advance();
                }
                '/' => {
                    tokens.push(Token::Slash);
                    self.advance();
                }
                '%' => {
                    tokens.push(Token::Percent);
                    self.advance();
                }
                '(' => {
                    tokens.push(Token::LeftParen);
                    self.advance();
                }
                ')' => {
                    tokens.push(Token::RightParen);
                    self.advance();
                }
                '[' => {
                    tokens.push(Token::LeftBracket);
                    self.advance();
                }
                ']' => {
                    tokens.push(Token::RightBracket);
                    self.advance();
                }
                '{' => {
                    tokens.push(Token::LeftBrace);
                    self.advance();
                }
                '}' => {
                    tokens.push(Token::RightBrace);
                    self.advance();
                }
                ',' => {
                    tokens.push(Token::Comma);
                    self.advance();
                }
                ':' => {
                    tokens.push(Token::Colon);
                    self.advance();
                }
                ';' => {
                    tokens.push(Token::Semicolon);
                    self.advance();
                }
                '.' => {
                    tokens.push(Token::Dot);
                    self.advance();
                }
                '=' => {
                    if self.peek(1) == Some('=') {
                        tokens.push(Token::EqualEqual);
                        self.advance();
                        self.advance();
                    } else {
                        tokens.push(Token::Equal);
                        self.advance();
                    }
                }
                '!' => {
                    if self.peek(1) == Some('=') {
                        tokens.push(Token::BangEqual);
                        self.advance();
                        self.advance();
                    } else {
                        tokens.push(Token::Not);
                        self.advance();
                    }
                }
                '<' => {
                    if self.peek(1) == Some('=') {
                        tokens.push(Token::LessEqual);
                        self.advance();
                        self.advance();
                    } else if self.peek(1) == Some('<') {
                        tokens.push(Token::LessLess);
                        self.advance();
                        self.advance();
                    } else {
                        tokens.push(Token::Less);
                        self.advance();
                    }
                }
                '>' => {
                    if self.peek(1) == Some('=') {
                        tokens.push(Token::GreaterEqual);
                        self.advance();
                        self.advance();
                    } else if self.peek(1) == Some('>') {
                        tokens.push(Token::GreaterGreater);
                        self.advance();
                        self.advance();
                    } else {
                        tokens.push(Token::Greater);
                        self.advance();
                    }
                }
                '&' => {
                    if self.peek(1) == Some('&') {
                        tokens.push(Token::And);
                        self.advance();
                        self.advance();
                    } else {
                        tokens.push(Token::Ampersand);
                        self.advance();
                    }
                }
                '|' => {
                    if self.peek(1) == Some('|') {
                        tokens.push(Token::Or);
                        self.advance();
                        self.advance();
                    } else {
                        tokens.push(Token::Pipe);
                        self.advance();
                    }
                }
                _ => self.advance(),
            }
        }

        tokens.push(Token::Eof);
        tokens
    }

    fn skip_comment(&mut self) {
        while self.current_char.is_some() && self.current_char != Some('\n') {
            self.advance();
        }
    }

    fn skip_multiline_comment(&mut self) {
        // Skip #[
        self.advance();
        self.advance();

        while let Some(ch) = self.current_char {
            if ch == ']' && self.peek(1) == Some('#') {
                self.advance(); // Skip ]
                self.advance(); // Skip #
                break;
            }
            self.advance();
        }
    }

    fn read_ui_sprite(&mut self) -> Token {
        // Skip ^÷^[
        self.advance();
        self.advance();
        self.advance();
        self.advance();

        let mut content = String::new();
        let mut depth = 1;

        while let Some(ch) = self.current_char {
            if ch == '[' {
                depth += 1;
                content.push(ch);
                self.advance();
            } else if ch == ']' {
                depth -= 1;
                if depth == 0 {
                    self.advance();
                    break;
                }
                content.push(ch);
                self.advance();
            } else {
                content.push(ch);
                self.advance();
            }
        }

        Token::UiSprite(content)
    }

    fn read_string(&mut self) -> Token {
        self.advance(); // Skip opening quote
        let mut value = String::new();

        while let Some(ch) = self.current_char {
            if ch == '"' {
                self.advance();
                break;
            }
            value.push(ch);
            self.advance();
        }

        Token::String(value)
    }

    fn read_number(&mut self) -> Token {
        let mut value = String::new();
        let mut is_float = false;

        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() {
                value.push(ch);
                self.advance();
            } else if ch == '.' && !is_float {
                is_float = true;
                value.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        if is_float {
            Token::Float(value.parse().unwrap())
        } else {
            Token::Integer(value.parse().unwrap())
        }
    }

    fn read_identifier_or_unit(&mut self) -> Token {
        let mut value = String::new();

        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' {
                value.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        // Check for keywords first
        let token = match value.as_str() {
            "imp" => Token::Imp,
            "def" => Token::Def,
            "fn" => Token::Fn,
            "asy" => Token::Asy,
            "cs" => Token::Cs,
            "mn" => Token::Mn,
            "own" => Token::Own,
            "ref" => Token::Ref,
            "copy" => Token::Copy,
            "await" => Token::Await,
            "loop" => Token::Loop,
            "if" => Token::If,
            "elif" => Token::Elif,
            "else" => Token::Else,
            "for" => Token::For,
            "while" => Token::While,
            "in" => Token::In,
            "return" => Token::Return,
            "break" => Token::Break,
            "continue" => Token::Continue,
            "try" => Token::Try,
            "catch" => Token::Catch,
            "true" => Token::Bool(true),
            "false" => Token::Bool(false),
            _ => Token::Identifier(value.clone()),
        };

        // If it's an identifier, check if it's followed by a unit pattern (e.g., m/s, kg)
        if matches!(token, Token::Identifier(_)) {
            // Check for unit patterns like m/s, m/s^2, kg, etc.
            if self.current_char == Some('/') || self.current_char == Some('^') {
                let mut unit = value.clone();

                // Read unit pattern
                while let Some(ch) = self.current_char {
                    if ch == '/' || ch == '^' || ch.is_alphanumeric() {
                        unit.push(ch);
                        self.advance();
                    } else {
                        break;
                    }
                }

                return Token::Unit(unit);
            }
        }

        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let mut lexer = Lexer::new("def x = 10");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Def);
        assert_eq!(tokens[1], Token::Identifier("x".to_string()));
        assert_eq!(tokens[2], Token::Equal);
        assert_eq!(tokens[3], Token::Integer(10));
    }

    #[test]
    fn test_ui_sprite() {
        let mut lexer = Lexer::new("def tree = ^÷^[tree]");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Def);
        assert_eq!(tokens[1], Token::Identifier("tree".to_string()));
        assert_eq!(tokens[2], Token::Equal);
        assert_eq!(tokens[3], Token::UiSprite("tree".to_string()));
    }

    #[test]
    fn test_ui_sprite_with_properties() {
        let mut lexer = Lexer::new("^÷^[slider{min=0, max=100}]");
        let tokens = lexer.tokenize();

        assert_eq!(
            tokens[0],
            Token::UiSprite("slider{min=0, max=100}".to_string())
        );
    }

    #[test]
    fn test_multiline_comment() {
        let mut lexer = Lexer::new("#[\nThis is a\nmulti-line comment\n]#\ndef x = 5");
        let tokens = lexer.tokenize();

        // Should skip the multi-line comment
        assert_eq!(tokens[0], Token::Newline);
        assert_eq!(tokens[1], Token::Def);
        assert_eq!(tokens[2], Token::Identifier("x".to_string()));
    }

    #[test]
    fn test_scientific_units() {
        let mut lexer = Lexer::new("def speed = 10 m/s");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Def);
        assert_eq!(tokens[1], Token::Identifier("speed".to_string()));
        assert_eq!(tokens[2], Token::Equal);
        assert_eq!(tokens[3], Token::Integer(10));
        assert_eq!(tokens[4], Token::Unit("m/s".to_string()));
    }

    #[test]
    fn test_scientific_units_complex() {
        let mut lexer = Lexer::new("def accel = 9.81 m/s^2");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Def);
        assert_eq!(tokens[1], Token::Identifier("accel".to_string()));
        assert_eq!(tokens[2], Token::Equal);
        assert_eq!(tokens[3], Token::Float(9.81));
        assert_eq!(tokens[4], Token::Unit("m/s^2".to_string()));
    }

    #[test]
    fn test_comparison_operators() {
        let mut lexer = Lexer::new("a == b != c < d <= e > f >= g");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Identifier("a".to_string()));
        assert_eq!(tokens[1], Token::EqualEqual);
        assert_eq!(tokens[2], Token::Identifier("b".to_string()));
        assert_eq!(tokens[3], Token::BangEqual);
        assert_eq!(tokens[4], Token::Identifier("c".to_string()));
        assert_eq!(tokens[5], Token::Less);
        assert_eq!(tokens[6], Token::Identifier("d".to_string()));
        assert_eq!(tokens[7], Token::LessEqual);
        assert_eq!(tokens[8], Token::Identifier("e".to_string()));
        assert_eq!(tokens[9], Token::Greater);
        assert_eq!(tokens[10], Token::Identifier("f".to_string()));
        assert_eq!(tokens[11], Token::GreaterEqual);
    }

    #[test]
    fn test_logical_operators() {
        let mut lexer = Lexer::new("a && b || !c");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Identifier("a".to_string()));
        assert_eq!(tokens[1], Token::And);
        assert_eq!(tokens[2], Token::Identifier("b".to_string()));
        assert_eq!(tokens[3], Token::Or);
        assert_eq!(tokens[4], Token::Not);
        assert_eq!(tokens[5], Token::Identifier("c".to_string()));
    }

    #[test]
    fn test_bitwise_operators() {
        let mut lexer = Lexer::new("a & b | c << d >> e");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Identifier("a".to_string()));
        assert_eq!(tokens[1], Token::Ampersand);
        assert_eq!(tokens[2], Token::Identifier("b".to_string()));
        assert_eq!(tokens[3], Token::Pipe);
        assert_eq!(tokens[4], Token::Identifier("c".to_string()));
        assert_eq!(tokens[5], Token::LessLess);
        assert_eq!(tokens[6], Token::Identifier("d".to_string()));
        assert_eq!(tokens[7], Token::GreaterGreater);
    }

    #[test]
    fn test_ownership_keywords() {
        let mut lexer = Lexer::new("own x ref y copy z");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Own);
        assert_eq!(tokens[1], Token::Identifier("x".to_string()));
        assert_eq!(tokens[2], Token::Ref);
        assert_eq!(tokens[3], Token::Identifier("y".to_string()));
        assert_eq!(tokens[4], Token::Copy);
        assert_eq!(tokens[5], Token::Identifier("z".to_string()));
    }

    #[test]
    fn test_async_await() {
        let mut lexer = Lexer::new("asy fetch():\n    res = await http.get(url)");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Asy);
        assert_eq!(tokens[1], Token::Identifier("fetch".to_string()));
        assert_eq!(tokens[2], Token::LeftParen);
        assert_eq!(tokens[3], Token::RightParen);
        assert_eq!(tokens[4], Token::Colon);
        assert_eq!(tokens[5], Token::Newline);
        // ... more tokens
        let await_pos = tokens.iter().position(|t| matches!(t, Token::Await));
        assert!(await_pos.is_some());
    }

    #[test]
    fn test_float_numbers() {
        let mut lexer = Lexer::new("3.14 2.718 0.5");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Float(3.14));
        assert_eq!(tokens[1], Token::Float(2.718));
        assert_eq!(tokens[2], Token::Float(0.5));
    }

    #[test]
    fn test_string_literals() {
        let mut lexer = Lexer::new("\"Hello, World!\" \"test\"");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::String("Hello, World!".to_string()));
        assert_eq!(tokens[1], Token::String("test".to_string()));
    }

    #[test]
    fn test_control_flow_keywords() {
        let mut lexer =
            Lexer::new("if x > 0:\n    return x\nelif x < 0:\n    return -x\nelse:\n    return 0");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::If);
        let elif_pos = tokens.iter().position(|t| matches!(t, Token::Elif));
        assert!(elif_pos.is_some());
        let else_pos = tokens.iter().position(|t| matches!(t, Token::Else));
        assert!(else_pos.is_some());
        let return_count = tokens.iter().filter(|t| matches!(t, Token::Return)).count();
        assert_eq!(return_count, 3);
    }
}
