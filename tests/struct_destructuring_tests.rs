use mirrow::library::ast::ASTNode;
use mirrow::library::lexer::{Lexer, Token, TokenKind, TokenValue};
use mirrow::library::parser::Parser;

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
fn test_struct_destructuring_parsing() {
    let source = "match obj { { name, age } -> name }";
    let nodes = parse_source(source).unwrap();
    
    assert_eq!(nodes.len(), 1);
    
    if let ASTNode::MatchStatement { value: _, arms } = &nodes[0] {
        assert_eq!(arms.len(), 1);
        
        let arm = &arms[0];
        assert_eq!(arm.patterns.len(), 1);
        
        // Check that it's a struct destructuring pattern
        if let ASTNode::StructDeconstructPattern { field_names } = &arm.patterns[0] {
            assert_eq!(field_names.len(), 2);
            
            if let TokenValue::Identifier(name) = &field_names[0].value {
                assert_eq!(name, "name");
            } else {
                panic!("Expected identifier token for first field");
            }
            
            if let TokenValue::Identifier(age) = &field_names[1].value {
                assert_eq!(age, "age");
            } else {
                panic!("Expected identifier token for second field");
            }
        } else {
            panic!("Expected struct destructuring pattern");
        }
    } else {
        panic!("Expected match statement");
    }
}

#[test]
fn test_struct_destructuring_single_field() {
    let source = "match user { { name } -> name }";
    let nodes = parse_source(source).unwrap();
    
    assert_eq!(nodes.len(), 1);
    
    if let ASTNode::MatchStatement { value: _, arms } = &nodes[0] {
        if let ASTNode::StructDeconstructPattern { field_names } = &arms[0].patterns[0] {
            assert_eq!(field_names.len(), 1);
            
            if let TokenValue::Identifier(name) = &field_names[0].value {
                assert_eq!(name, "name");
            }
        } else {
            panic!("Expected struct destructuring pattern");
        }
    }
}

#[test]
fn test_struct_destructuring_multiple_fields() {
    let source = "match person { { name, age, city, country } -> name }";
    let nodes = parse_source(source).unwrap();
    
    assert_eq!(nodes.len(), 1);
    
    if let ASTNode::MatchStatement { value: _, arms } = &nodes[0] {
        if let ASTNode::StructDeconstructPattern { field_names } = &arms[0].patterns[0] {
            assert_eq!(field_names.len(), 4);
            
            let expected_fields = vec!["name", "age", "city", "country"];
            for (i, expected) in expected_fields.iter().enumerate() {
                if let TokenValue::Identifier(actual) = &field_names[i].value {
                    assert_eq!(actual, expected);
                } else {
                    panic!("Expected identifier token for field {}", i);
                }
            }
        } else {
            panic!("Expected struct destructuring pattern");
        }
    }
}

#[test]
fn test_struct_destructuring_with_wildcard() {
    let source = "match data { { name, age } -> name, _ -> \"unknown\" }";
    let nodes = parse_source(source).unwrap();
    
    assert_eq!(nodes.len(), 1);
    
    if let ASTNode::MatchStatement { value: _, arms } = &nodes[0] {
        assert_eq!(arms.len(), 2);
        
        // First arm should be struct destructuring
        matches!(arms[0].patterns[0], ASTNode::StructDeconstructPattern { .. });
        
        // Second arm should be wildcard
        matches!(arms[1].patterns[0], ASTNode::WildcardPattern);
    } else {
        panic!("Expected match statement");
    }
}

#[test]
fn test_struct_destructuring_or_operator_rejection() {
    // This should fail to parse due to OR operator restriction
    let source = "match obj { { name } | { age } -> \"either\" }";
    let result = parse_source(source);
    
    assert!(result.is_err(), "Expected parsing to fail due to OR operator with struct patterns");
}

#[test]
fn test_struct_destructuring_or_operator_rejection_mixed() {
    // This should fail: struct pattern mixed with other pattern using OR
    let source = "match obj { { name } | 42 -> \"mixed\" }";
    let result = parse_source(source);
    
    assert!(result.is_err(), "Expected parsing to fail due to OR operator with struct patterns");
}

#[test]
fn test_struct_destructuring_or_operator_rejection_reverse() {
    // This should also fail: other pattern mixed with struct pattern using OR
    let source = "match obj { 42 | { name } -> \"mixed\" }";
    let result = parse_source(source);
    
    assert!(result.is_err(), "Expected parsing to fail due to OR operator with struct patterns");
}

#[test]
fn test_empty_struct_destructuring_rejection() {
    // This should fail - empty struct patterns not allowed
    let source = "match obj { {} -> \"empty\" }";
    let result = parse_source(source);
    
    assert!(result.is_err(), "Expected parsing to fail for empty struct pattern");
}

#[test]
fn test_struct_pattern_vs_struct_literal() {
    // Test that struct literals in expressions still work
    let source = "let obj = { name = \"John\", age = 30 }";
    let nodes = parse_source(source).unwrap();
    
    assert_eq!(nodes.len(), 1);
    
    if let ASTNode::LetStatement { initializer, .. } = &nodes[0] {
        matches!(initializer.as_ref(), ASTNode::StructLiteral { .. });
    } else {
        panic!("Expected let statement with struct literal");
    }
}

#[test]
fn test_nested_struct_destructuring_in_multiple_arms() {
    let source = r#"
        match result {
            { success = true, data } -> data,
            { success = false, error } -> error,
            _ -> "unknown"
        }
    "#;
    
    // This should fail because struct patterns can't have field assignments
    let result = parse_source(source);
    assert!(result.is_err(), "Expected parsing to fail for struct patterns with field assignments");
}

#[test]
fn test_complex_match_with_struct_destructuring() {
    let source = r#"
        match response {
            { status } -> status,
            1 -> "number_one",
            "error" -> "string_error",
            _ -> "fallback"
        }
    "#;
    
    let nodes = parse_source(source).unwrap();
    assert_eq!(nodes.len(), 1);
    
    if let ASTNode::MatchStatement { arms, .. } = &nodes[0] {
        assert_eq!(arms.len(), 4);
        
        // First arm: struct destructuring
        matches!(arms[0].patterns[0], ASTNode::StructDeconstructPattern { .. });
        
        // Second arm: number literal
        matches!(arms[1].patterns[0], ASTNode::Literal { .. });
        
        // Third arm: string literal  
        matches!(arms[2].patterns[0], ASTNode::Literal { .. });
        
        // Fourth arm: wildcard
        matches!(arms[3].patterns[0], ASTNode::WildcardPattern);
    } else {
        panic!("Expected match statement");
    }
}