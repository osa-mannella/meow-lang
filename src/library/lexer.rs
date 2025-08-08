#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Identifier,
    Number,
    String,
    InterpolatedString,

    Let,
    Func,
    If,
    Else,
    True,
    False,
    Match,
    Fn,
    Async,
    Await,
    Import,
    Enum,
    Power,

    Equal,
    EqualEqual,
    BangEqual,
    GreaterEqual,
    LessEqual,
    Greater,
    Less,
    Plus,
    Minus,
    Star,
    Slash,
    Comma,
    Semicolon,
    Colon,
    DoubleColon,
    Bang,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    LParen,
    RParen,
    Dot,
    And,
    Or,
    Arrow,
    Question,
    Reflect,
    Pipe,
    Pipeline,
    LArrow,
    Dollar,

    Eof,
    Error,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenValue {
    None,
    Identifier(String),
    Number(f64),
    String(String),
    Error(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: TokenValue,
    pub line: usize,
}

impl Token {
    pub fn eof() -> Self {
        Token {
            kind: TokenKind::Eof,
            value: TokenValue::None,
            line: 0,
        }
    }
}

pub struct Lexer<'a> {
    chars: std::str::Chars<'a>,
    current: Option<char>,
    line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut chars = source.chars();
        let current = chars.next();
        Self {
            chars,
            current,
            line: 1,
        }
    }

    fn advance(&mut self) -> Option<char> {
        match self.current {
            Some('\n') => {
                self.line += 1;
            }
            _ => {}
        }
        self.current = self.chars.next();
        self.current
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.as_str().chars().next()
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.current == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn skip_comments(&mut self) -> bool {
        match self.current {
            Some('/') => {
                match self.peek() {
                    Some('/') => {
                        self.advance(); // consume first /
                        self.advance(); // consume second /
                        while let Some(ch) = self.current {
                            if ch == '\n' {
                                break;
                            }
                            self.advance();
                        }
                        true
                    }
                    Some('*') => {
                        self.advance(); // consume /
                        self.advance(); // consume *
                        while let Some(ch) = self.current {
                            if ch == '*' && self.peek() == Some('/') {
                                self.advance(); // consume *
                                self.advance(); // consume /
                                break;
                            }
                            self.advance();
                        }
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    fn make_number(&mut self) -> Option<Token> {
        let mut number = String::new();

        while let Some(ch) = self.current {
            if ch.is_ascii_digit() || ch == '.' {
                number.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        if let Ok(value) = number.parse::<f64>() {
            Some(Token {
                kind: TokenKind::Number,
                value: TokenValue::Number(value),
                line: self.line,
            })
        } else {
            Some(Token {
                kind: TokenKind::Error,
                value: TokenValue::Error(format!("Invalid number: {}", number)),
                line: self.line,
            })
        }
    }

    fn make_string(&mut self) -> Option<Token> {
        self.make_regular_string()
    }

    fn make_regular_string(&mut self) -> Option<Token> {
        let mut string = String::new();
        self.advance(); // consume opening quote

        while let Some(ch) = self.current {
            if ch == '"' {
                self.advance(); // consume closing quote
                return Some(Token {
                    kind: TokenKind::String,
                    value: TokenValue::String(string),
                    line: self.line,
                });
            } else if ch == '\\' {
                self.advance();
                if let Some(escaped) = self.current {
                    match escaped {
                        'n' => string.push('\n'),
                        't' => string.push('\t'),
                        'r' => string.push('\r'),
                        '\\' => string.push('\\'),
                        '"' => string.push('"'),
                        _ => {
                            string.push('\\');
                            string.push(escaped);
                        }
                    }
                    self.advance();
                }
            } else {
                string.push(ch);
                self.advance();
            }
        }

        Some(Token {
            kind: TokenKind::Error,
            value: TokenValue::Error("Unterminated string".to_string()),
            line: self.line,
        })
    }

    fn make_interpolated_string(&mut self) -> Option<Token> {
        let mut string = String::new();
        self.advance(); // consume opening quote

        while let Some(ch) = self.current {
            if ch == '"' {
                self.advance(); // consume closing quote
                return Some(Token {
                    kind: TokenKind::InterpolatedString,
                    value: TokenValue::String(string),
                    line: self.line,
                });
            } else if ch == '\\' {
                self.advance();
                if let Some(escaped) = self.current {
                    match escaped {
                        'n' => {
                            string.push('\\');
                            string.push('n');
                        },
                        't' => {
                            string.push('\\');
                            string.push('t');
                        },
                        'r' => {
                            string.push('\\');
                            string.push('r');
                        },
                        '\\' => {
                            string.push('\\');
                            string.push('\\');
                        },
                        '"' => {
                            string.push('\\');
                            string.push('"');
                        },
                        '$' => {
                            string.push('\\');
                            string.push('$');
                        },
                        _ => {
                            string.push('\\');
                            string.push(escaped);
                        }
                    }
                    self.advance();
                }
            } else {
                string.push(ch);
                self.advance();
            }
        }

        Some(Token {
            kind: TokenKind::Error,
            value: TokenValue::Error("Unterminated interpolated string".to_string()),
            line: self.line,
        })
    }

    fn make_identifier(&mut self) -> Option<Token> {
        let mut identifier = String::new();

        while let Some(ch) = self.current {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                identifier.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        let kind = match identifier.as_str() {
            "let" => TokenKind::Let,
            "func" => TokenKind::Func,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "match" => TokenKind::Match,
            "fn" => TokenKind::Fn,
            "async" => TokenKind::Async,
            "await" => TokenKind::Await,
            "import" => TokenKind::Import,
            "enum" => TokenKind::Enum,
            _ => TokenKind::Identifier,
        };

        Some(Token {
            kind: kind.clone(),
            value: if kind == TokenKind::Identifier {
                TokenValue::Identifier(identifier)
            } else {
                TokenValue::None
            },
            line: self.line,
        })
    }

    pub fn next(&mut self) -> Option<Token> {
        loop {
            self.skip_whitespace();
            if !self.skip_comments() {
                break;
            }
        }

        let ch = self.current?;

        match ch {
            // Single character tokens
            '(' => {
                self.advance();
                Some(Token {
                    kind: TokenKind::LParen,
                    value: TokenValue::None,
                    line: self.line,
                })
            }
            ')' => {
                self.advance();
                Some(Token {
                    kind: TokenKind::RParen,
                    value: TokenValue::None,
                    line: self.line,
                })
            }
            '{' => {
                self.advance();
                Some(Token {
                    kind: TokenKind::LBrace,
                    value: TokenValue::None,
                    line: self.line,
                })
            }
            '}' => {
                self.advance();
                Some(Token {
                    kind: TokenKind::RBrace,
                    value: TokenValue::None,
                    line: self.line,
                })
            }
            '[' => {
                self.advance();
                Some(Token {
                    kind: TokenKind::LBracket,
                    value: TokenValue::None,
                    line: self.line,
                })
            }
            ']' => {
                self.advance();
                Some(Token {
                    kind: TokenKind::RBracket,
                    value: TokenValue::None,
                    line: self.line,
                })
            }
            ',' => {
                self.advance();
                Some(Token {
                    kind: TokenKind::Comma,
                    value: TokenValue::None,
                    line: self.line,
                })
            }
            ';' => {
                self.advance();
                Some(Token {
                    kind: TokenKind::Semicolon,
                    value: TokenValue::None,
                    line: self.line,
                })
            }
            '+' => {
                self.advance();
                Some(Token {
                    kind: TokenKind::Plus,
                    value: TokenValue::None,
                    line: self.line,
                })
            }
            '-' => {
                self.advance();
                if self.match_char('>') {
                    Some(Token {
                        kind: TokenKind::Arrow,
                        value: TokenValue::None,
                        line: self.line,
                    })
                } else {
                    Some(Token {
                        kind: TokenKind::Minus,
                        value: TokenValue::None,
                        line: self.line,
                    })
                }
            }
            '*' => {
                self.advance();
                if self.match_char('*') {
                    Some(Token {
                        kind: TokenKind::Power,
                        value: TokenValue::None,
                        line: self.line,
                    })
                } else {
                    Some(Token {
                        kind: TokenKind::Star,
                        value: TokenValue::None,
                        line: self.line,
                    })
                }
            }
            '/' => {
                self.advance();
                Some(Token {
                    kind: TokenKind::Slash,
                    value: TokenValue::None,
                    line: self.line,
                })
            }
            '.' => {
                self.advance();
                Some(Token {
                    kind: TokenKind::Dot,
                    value: TokenValue::None,
                    line: self.line,
                })
            }
            '?' => {
                self.advance();
                Some(Token {
                    kind: TokenKind::Question,
                    value: TokenValue::None,
                    line: self.line,
                })
            }
            '$' => {
                if self.peek() == Some('"') {
                    self.advance(); // consume '$'
                    self.make_interpolated_string()
                } else {
                    self.advance();
                    Some(Token {
                        kind: TokenKind::Dollar,
                        value: TokenValue::None,
                        line: self.line,
                    })
                }
            }

            // Multi-character tokens
            '=' => {
                self.advance();
                if self.match_char('=') {
                    Some(Token {
                        kind: TokenKind::EqualEqual,
                        value: TokenValue::None,
                        line: self.line,
                    })
                } else {
                    Some(Token {
                        kind: TokenKind::Equal,
                        value: TokenValue::None,
                        line: self.line,
                    })
                }
            }
            '!' => {
                self.advance();
                if self.match_char('=') {
                    Some(Token {
                        kind: TokenKind::BangEqual,
                        value: TokenValue::None,
                        line: self.line,
                    })
                } else {
                    Some(Token {
                        kind: TokenKind::Bang,
                        value: TokenValue::None,
                        line: self.line,
                    })
                }
            }
            '<' => {
                self.advance();
                if self.match_char('=') {
                    Some(Token {
                        kind: TokenKind::LessEqual,
                        value: TokenValue::None,
                        line: self.line,
                    })
                } else if self.match_char('-') {
                    Some(Token {
                        kind: TokenKind::LArrow,
                        value: TokenValue::None,
                        line: self.line,
                    })
                } else {
                    Some(Token {
                        kind: TokenKind::Less,
                        value: TokenValue::None,
                        line: self.line,
                    })
                }
            }
            '>' => {
                self.advance();
                if self.match_char('=') {
                    Some(Token {
                        kind: TokenKind::GreaterEqual,
                        value: TokenValue::None,
                        line: self.line,
                    })
                } else {
                    Some(Token {
                        kind: TokenKind::Greater,
                        value: TokenValue::None,
                        line: self.line,
                    })
                }
            }
            ':' => {
                self.advance();
                if self.match_char(':') {
                    Some(Token {
                        kind: TokenKind::DoubleColon,
                        value: TokenValue::None,
                        line: self.line,
                    })
                } else {
                    Some(Token {
                        kind: TokenKind::Colon,
                        value: TokenValue::None,
                        line: self.line,
                    })
                }
            }
            '&' => {
                self.advance();
                if self.match_char('&') {
                    Some(Token {
                        kind: TokenKind::And,
                        value: TokenValue::None,
                        line: self.line,
                    })
                } else {
                    Some(Token {
                        kind: TokenKind::Reflect,
                        value: TokenValue::None,
                        line: self.line,
                    })
                }
            }
            '|' => {
                self.advance();
                if self.match_char('|') {
                    Some(Token {
                        kind: TokenKind::Or,
                        value: TokenValue::None,
                        line: self.line,
                    })
                } else if self.match_char('>') {
                    Some(Token {
                        kind: TokenKind::Pipeline,
                        value: TokenValue::None,
                        line: self.line,
                    })
                } else {
                    Some(Token {
                        kind: TokenKind::Pipe,
                        value: TokenValue::None,
                        line: self.line,
                    })
                }
            }

            // String literals
            '"' => self.make_string(),

            // Numbers
            ch if ch.is_ascii_digit() => self.make_number(),

            // Identifiers and keywords
            ch if ch.is_ascii_alphabetic() || ch == '_' => self.make_identifier(),

            _ => {
                self.advance();
                Some(Token {
                    kind: TokenKind::Error,
                    value: TokenValue::Error(format!("Unexpected character: {}", ch)),
                    line: self.line,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tokenize_all(source: &str) -> Vec<Token> {
        let mut lexer = Lexer::new(source);
        let mut tokens = Vec::new();

        while let Some(token) = lexer.next() {
            let is_eof = token.kind == TokenKind::Eof;
            tokens.push(token);
            if is_eof {
                break;
            }
        }

        tokens
    }

    #[test]
    fn test_single_character_tokens() {
        let source = "(){}[],;+-*/.?$_";
        let tokens = tokenize_all(source);

        let expected_kinds = vec![
            TokenKind::LParen,
            TokenKind::RParen,
            TokenKind::LBrace,
            TokenKind::RBrace,
            TokenKind::LBracket,
            TokenKind::RBracket,
            TokenKind::Comma,
            TokenKind::Semicolon,
            TokenKind::Plus,
            TokenKind::Minus,
            TokenKind::Star,
            TokenKind::Slash,
            TokenKind::Dot,
            TokenKind::Question,
            TokenKind::Dollar,
            TokenKind::Identifier,
        ];

        for (i, expected_kind) in expected_kinds.iter().enumerate() {
            assert_eq!(tokens[i].kind, *expected_kind, "Token {} mismatch", i);
        }
    }

    #[test]
    fn test_multi_character_operators() {
        let source = "== != <= >= <- -> :: && || |> **";
        let tokens = tokenize_all(source);

        let expected_kinds = vec![
            TokenKind::EqualEqual,
            TokenKind::BangEqual,
            TokenKind::LessEqual,
            TokenKind::GreaterEqual,
            TokenKind::LArrow,
            TokenKind::Arrow,
            TokenKind::DoubleColon,
            TokenKind::And,
            TokenKind::Or,
            TokenKind::Pipeline,
            TokenKind::Power,
        ];

        for (i, expected_kind) in expected_kinds.iter().enumerate() {
            assert_eq!(tokens[i].kind, *expected_kind, "Token {} mismatch", i);
        }
    }

    #[test]
    fn test_keywords() {
        let source = "let func if else true false match fn async await import enum";
        let tokens = tokenize_all(source);

        let expected_kinds = vec![
            TokenKind::Let,
            TokenKind::Func,
            TokenKind::If,
            TokenKind::Else,
            TokenKind::True,
            TokenKind::False,
            TokenKind::Match,
            TokenKind::Fn,
            TokenKind::Async,
            TokenKind::Await,
            TokenKind::Import,
            TokenKind::Enum,
        ];

        for (i, expected_kind) in expected_kinds.iter().enumerate() {
            assert_eq!(tokens[i].kind, *expected_kind, "Keyword {} mismatch", i);
        }
    }

    #[test]
    fn test_identifiers() {
        let source = "hello world my_variable _private var123";
        let tokens = tokenize_all(source);

        let expected_values = vec!["hello", "world", "my_variable", "_private", "var123"];

        for (i, expected_value) in expected_values.iter().enumerate() {
            assert_eq!(tokens[i].kind, TokenKind::Identifier);
            if let TokenValue::Identifier(name) = &tokens[i].value {
                assert_eq!(name, expected_value, "Identifier {} value mismatch", i);
            } else {
                panic!("Expected identifier token value at position {}", i);
            }
        }
    }

    #[test]
    fn test_numbers() {
        let source = "42 3.14 0 123.456";
        let tokens = tokenize_all(source);

        let expected_values = vec![42.0, 3.14, 0.0, 123.456];

        for (i, expected_value) in expected_values.iter().enumerate() {
            assert_eq!(tokens[i].kind, TokenKind::Number);
            if let TokenValue::Number(value) = tokens[i].value {
                assert!(
                    (value - expected_value).abs() < f64::EPSILON,
                    "Number {} value mismatch: expected {}, got {}",
                    i,
                    expected_value,
                    value
                );
            } else {
                panic!("Expected number token value at position {}", i);
            }
        }
    }

    #[test]
    fn test_strings() {
        let source = r#""hello" "world with spaces" "escaped \"quote\"" "newline\n""#;
        let tokens = tokenize_all(source);

        let expected_values = vec![
            "hello",
            "world with spaces",
            "escaped \"quote\"",
            "newline\n",
        ];

        for (i, expected_value) in expected_values.iter().enumerate() {
            assert_eq!(tokens[i].kind, TokenKind::String);
            if let TokenValue::String(value) = &tokens[i].value {
                assert_eq!(value, expected_value, "String {} value mismatch", i);
            } else {
                panic!("Expected string token value at position {}", i);
            }
        }
    }

    #[test]
    fn test_complex_expression() {
        let source = "let result = my_func(x, y) + 42 * 3.14";
        let tokens = tokenize_all(source);

        let expected_kinds = vec![
            TokenKind::Let,
            TokenKind::Identifier,
            TokenKind::Equal,
            TokenKind::Identifier,
            TokenKind::LParen,
            TokenKind::Identifier,
            TokenKind::Comma,
            TokenKind::Identifier,
            TokenKind::RParen,
            TokenKind::Plus,
            TokenKind::Number,
            TokenKind::Star,
            TokenKind::Number,
        ];

        for (i, expected_kind) in expected_kinds.iter().enumerate() {
            assert_eq!(
                tokens[i].kind, *expected_kind,
                "Complex expression token {} mismatch",
                i
            );
        }
    }

    #[test]
    fn test_array_append_syntax() {
        let source = "arr <- [1, 2, 3]";
        let tokens = tokenize_all(source);

        let expected_kinds = vec![
            TokenKind::Identifier,
            TokenKind::LArrow,
            TokenKind::LBracket,
            TokenKind::Number,
            TokenKind::Comma,
            TokenKind::Number,
            TokenKind::Comma,
            TokenKind::Number,
            TokenKind::RBracket,
        ];

        for (i, expected_kind) in expected_kinds.iter().enumerate() {
            assert_eq!(
                tokens[i].kind, *expected_kind,
                "Array append token {} mismatch",
                i
            );
        }
    }

    #[test]
    fn test_struct_syntax() {
        let source = "Person::Programmer { name = \"John\", age = 30 }";
        let tokens = tokenize_all(source);

        let expected_kinds = vec![
            TokenKind::Identifier,
            TokenKind::DoubleColon,
            TokenKind::Identifier,
            TokenKind::LBrace,
            TokenKind::Identifier,
            TokenKind::Equal,
            TokenKind::String,
            TokenKind::Comma,
            TokenKind::Identifier,
            TokenKind::Equal,
            TokenKind::Number,
            TokenKind::RBrace,
        ];

        for (i, expected_kind) in expected_kinds.iter().enumerate() {
            assert_eq!(
                tokens[i].kind, *expected_kind,
                "Struct syntax token {} mismatch",
                i
            );
        }
    }

    #[test]
    fn test_match_expression() {
        let source = "match value { Some(x) -> x, None -> 0 }";
        let tokens = tokenize_all(source);

        let expected_kinds = vec![
            TokenKind::Match,
            TokenKind::Identifier,
            TokenKind::LBrace,
            TokenKind::Identifier,
            TokenKind::LParen,
            TokenKind::Identifier,
            TokenKind::RParen,
            TokenKind::Arrow,
            TokenKind::Identifier,
            TokenKind::Comma,
            TokenKind::Identifier,
            TokenKind::Arrow,
            TokenKind::Number,
            TokenKind::RBrace,
        ];

        for (i, expected_kind) in expected_kinds.iter().enumerate() {
            assert_eq!(
                tokens[i].kind, *expected_kind,
                "Match expression token {} mismatch",
                i
            );
        }
    }

    #[test]
    fn test_async_await_syntax() {
        let source = "async func test() { await some_async_call() }";
        let tokens = tokenize_all(source);

        let expected_kinds = vec![
            TokenKind::Async,
            TokenKind::Func,
            TokenKind::Identifier,
            TokenKind::LParen,
            TokenKind::RParen,
            TokenKind::LBrace,
            TokenKind::Await,
            TokenKind::Identifier,
            TokenKind::LParen,
            TokenKind::RParen,
            TokenKind::RBrace,
        ];

        for (i, expected_kind) in expected_kinds.iter().enumerate() {
            assert_eq!(
                tokens[i].kind, *expected_kind,
                "Async/await token {} mismatch",
                i
            );
        }
    }

    #[test]
    fn test_pipeline_operator() {
        let source = "value |> transform |> filter";
        let tokens = tokenize_all(source);

        let expected_kinds = vec![
            TokenKind::Identifier,
            TokenKind::Pipeline,
            TokenKind::Identifier,
            TokenKind::Pipeline,
            TokenKind::Identifier,
        ];

        for (i, expected_kind) in expected_kinds.iter().enumerate() {
            assert_eq!(
                tokens[i].kind, *expected_kind,
                "Pipeline token {} mismatch",
                i
            );
        }
    }

    #[test]
    fn test_line_numbers() {
        let source = "let\nx\n=\n42";
        let tokens = tokenize_all(source);

        assert_eq!(tokens[0].line, 1); // let
        assert_eq!(tokens[1].line, 2); // x  
        assert_eq!(tokens[2].line, 3); // =
        assert_eq!(tokens[3].line, 4); // 42
    }

    #[test]
    fn test_whitespace_handling() {
        let source = "  let   x    =    42   ";
        let tokens = tokenize_all(source);

        let expected_kinds = vec![
            TokenKind::Let,
            TokenKind::Identifier,
            TokenKind::Equal,
            TokenKind::Number,
        ];

        for (i, expected_kind) in expected_kinds.iter().enumerate() {
            assert_eq!(
                tokens[i].kind, *expected_kind,
                "Whitespace handling token {} mismatch",
                i
            );
        }
    }

    #[test]
    fn test_error_handling() {
        let source = "@#%";
        let tokens = tokenize_all(source);

        // Should produce Error tokens for unexpected characters
        for token in &tokens[..3] {
            // Skip EOF
            assert_eq!(token.kind, TokenKind::Error);
        }
    }

    #[test]
    fn test_unterminated_string() {
        let source = r#""unterminated string"#;
        let tokens = tokenize_all(source);

        assert_eq!(tokens[0].kind, TokenKind::Error);
        if let TokenValue::Error(msg) = &tokens[0].value {
            assert!(msg.contains("Unterminated string"));
        }
    }

    #[test]
    fn test_empty_input() {
        let source = "";
        let tokens = tokenize_all(source);

        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn test_whitespace_only() {
        let source = "   \n  \t  \n  ";
        let tokens = tokenize_all(source);

        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn test_interpolated_strings() {
        let source = r#"$"Hello ${name}!""#;
        let tokens = tokenize_all(source);

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::InterpolatedString);
        if let TokenValue::String(s) = &tokens[0].value {
            assert_eq!(s, "Hello ${name}!");
        } else {
            panic!("Expected string value in interpolated string token");
        }
    }

    #[test]
    fn test_interpolated_strings_multiple_expressions() {
        let source = r#"$"User ${user} has ${count} items""#;
        let tokens = tokenize_all(source);

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::InterpolatedString);
        if let TokenValue::String(s) = &tokens[0].value {
            assert_eq!(s, "User ${user} has ${count} items");
        } else {
            panic!("Expected string value in interpolated string token");
        }
    }
}
