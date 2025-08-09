use mirrow::library::ast::ASTNode;
use mirrow::library::lexer::{Lexer, Token, TokenKind, TokenValue};
use mirrow::library::parser::Parser;

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

fn parse_source(source: &str) -> Result<Vec<ASTNode>, String> {
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    if parser.had_error {
        Err("Parser encountered errors".to_string())
    } else {
        Ok(program.nodes)
    }
}

#[test]
fn test_underscore_wildcard_vs_identifier() {
    // Test standalone underscore (wildcard)
    let source = "_ + _foo + _ + _bar_";
    let tokens = tokenize_all(source);

    let expected_kinds = vec![
        TokenKind::Underscore, // standalone _
        TokenKind::Plus,
        TokenKind::Identifier, // _foo
        TokenKind::Plus,
        TokenKind::Underscore, // standalone _
        TokenKind::Plus,
        TokenKind::Identifier, // _bar_
    ];

    for (i, expected_kind) in expected_kinds.iter().enumerate() {
        assert_eq!(
            tokens[i].kind, *expected_kind,
            "Underscore token {} mismatch: expected {:?}, got {:?}",
            i, expected_kind, tokens[i].kind
        );
    }

    // Verify identifier values
    if let TokenValue::Identifier(name) = &tokens[2].value {
        assert_eq!(name, "_foo");
    }
    if let TokenValue::Identifier(name) = &tokens[6].value {
        assert_eq!(name, "_bar_");
    }
}

#[test]
fn test_underscore_in_match_pattern() {
    let source = "match x { 1 -> \"one\", _ -> \"other\" }";
    let tokens = tokenize_all(source);

    let expected_kinds = vec![
        TokenKind::Match,
        TokenKind::Identifier,
        TokenKind::LBrace,
        TokenKind::Number,
        TokenKind::Arrow,
        TokenKind::String,
        TokenKind::Comma,
        TokenKind::Underscore, // wildcard pattern
        TokenKind::Arrow,
        TokenKind::String,
        TokenKind::RBrace,
    ];

    for (i, expected_kind) in expected_kinds.iter().enumerate() {
        assert_eq!(
            tokens[i].kind, *expected_kind,
            "Match with underscore token {} mismatch: expected {:?}, got {:?}",
            i, expected_kind, tokens[i].kind
        );
    }
}

#[test]
fn test_underscore_at_end_of_input() {
    let source = "let x = _";
    let tokens = tokenize_all(source);

    let expected_kinds = vec![
        TokenKind::Let,
        TokenKind::Identifier,
        TokenKind::Equal,
        TokenKind::Underscore,
    ];

    for (i, expected_kind) in expected_kinds.iter().enumerate() {
        assert_eq!(
            tokens[i].kind, *expected_kind,
            "End of input underscore token {} mismatch",
            i
        );
    }
}

#[test]
fn test_underscore_parsing_in_match() {
    let source = "match value { 1 -> \"one\", 2 -> \"two\", _ -> \"default\" }";
    let nodes = parse_source(source).unwrap();

    assert_eq!(nodes.len(), 1);

    if let ASTNode::MatchStatement { value: _, arms } = &nodes[0] {
        assert_eq!(arms.len(), 3);

        // Check that the third arm has a wildcard pattern
        let wildcard_arm = &arms[2];
        assert_eq!(wildcard_arm.patterns.len(), 1);

        matches!(wildcard_arm.patterns[0], ASTNode::WildcardPattern);
    } else {
        panic!("Expected match statement");
    }
}

#[test]
fn test_underscore_mixed_patterns() {
    let source = "match result { 1 -> \"one\", 2 -> \"two\", _ -> \"default\" }";
    let nodes = parse_source(source).unwrap();

    assert_eq!(nodes.len(), 1);

    if let ASTNode::MatchStatement { value: _, arms } = &nodes[0] {
        assert_eq!(arms.len(), 3);

        // First two arms: literal patterns
        matches!(arms[0].patterns[0], ASTNode::Literal { .. });
        matches!(arms[1].patterns[0], ASTNode::Literal { .. });

        // Third arm: wildcard pattern
        matches!(arms[2].patterns[0], ASTNode::WildcardPattern);
    } else {
        panic!("Expected match statement");
    }
}

#[test]
fn test_multiple_underscores() {
    let source = "_ _ _";
    let tokens = tokenize_all(source);

    let expected_kinds = vec![
        TokenKind::Underscore,
        TokenKind::Underscore,
        TokenKind::Underscore,
    ];

    for (i, expected_kind) in expected_kinds.iter().enumerate() {
        assert_eq!(
            tokens[i].kind, *expected_kind,
            "Multiple underscores token {} mismatch",
            i
        );
    }
}

#[test]
fn test_underscore_with_whitespace() {
    let source = " _ ";
    let tokens = tokenize_all(source);

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Underscore);
}
