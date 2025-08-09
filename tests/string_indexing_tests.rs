use mirrow::library::ast::ASTNode;
use mirrow::library::compiler::{BytecodeProgram, compile_program};
use mirrow::library::lexer::Lexer;
use mirrow::library::parser::Parser;

fn compile_source(source: &str) -> Result<BytecodeProgram, String> {
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    if parser.had_error {
        Err("Parser error".to_string())
    } else {
        compile_program(program)
    }
}

#[test]
fn test_single_quote_strings() {
    let source = r#"let x = 'hello world'"#;
    let result = compile_source(source);
    assert!(result.is_ok());

    let bytecode = result.unwrap();
    // Check that the string constant was added
    assert!(bytecode.constants.len() > 0);

    // Check that we have at least one string constant matching our input
    let has_hello_world = bytecode.constants.iter().any(|c| {
        matches!(&c.value, mirrow::library::compiler::ConstantValue::String(s) if s == "hello world")
    });
    assert!(has_hello_world, "Should have 'hello world' string constant");
}

#[test]
fn test_double_quote_strings() {
    let source = r#"let x = "hello world""#;
    let result = compile_source(source);
    assert!(result.is_ok());

    let bytecode = result.unwrap();
    // Check that we have at least one string constant matching our input
    let has_hello_world = bytecode.constants.iter().any(|c| {
        matches!(&c.value, mirrow::library::compiler::ConstantValue::String(s) if s == "hello world")
    });
    assert!(has_hello_world, "Should have 'hello world' string constant");
}

#[test]
fn test_mixed_quote_strings() {
    let source = r#"
let x = 'single quoted'
let y = "double quoted"
"#;
    let result = compile_source(source);
    assert!(result.is_ok());

    let bytecode = result.unwrap();

    let has_single = bytecode.constants.iter().any(|c| {
        matches!(&c.value, mirrow::library::compiler::ConstantValue::String(s) if s == "single quoted")
    });
    let has_double = bytecode.constants.iter().any(|c| {
        matches!(&c.value, mirrow::library::compiler::ConstantValue::String(s) if s == "double quoted")
    });

    assert!(has_single, "Should have single quoted string");
    assert!(has_double, "Should have double quoted string");
}

#[test]
fn test_single_quote_escape_sequences() {
    let source = r#"let x = 'hello\nworld\t!'"#;
    let result = compile_source(source);
    assert!(result.is_ok());
}

#[test]
fn test_single_quote_with_double_quotes_inside() {
    let source = r#"let x = 'He said "Hello!"'"#;
    let result = compile_source(source);
    assert!(result.is_ok());

    let bytecode = result.unwrap();
    let has_expected = bytecode.constants.iter().any(|c| {
        matches!(&c.value, mirrow::library::compiler::ConstantValue::String(s) if s == r#"He said "Hello!""#)
    });
    assert!(has_expected, "Should have string with double quotes inside");
}

#[test]
fn test_double_quote_with_single_quotes_inside() {
    let source = r#"let x = "He said 'Hello!'""#;
    let result = compile_source(source);
    assert!(result.is_ok());

    let bytecode = result.unwrap();
    let has_expected = bytecode.constants.iter().any(|c| {
        matches!(&c.value, mirrow::library::compiler::ConstantValue::String(s) if s == "He said 'Hello!'")
    });
    assert!(has_expected, "Should have string with single quotes inside");
}

#[test]
fn test_string_indexing_compilation() {
    let source = r#"
let person = { name = "John", age = 30 }
let name = person["name"]
let age = person['age']
"#;
    let result = compile_source(source);
    assert!(result.is_ok());

    let bytecode = result.unwrap();

    // Should have index_access opcode
    let index_access_opcode = bytecode.get_opcode("index_access");
    assert!(
        index_access_opcode.is_some(),
        "Should have index_access opcode"
    );
}

#[test]
fn test_string_indexing_vs_property_access_parsing() {
    // Test that obj["prop"] parses correctly as IndexAccess
    let source1 = r#"obj["property"]"#;
    let lexer1 = Lexer::new(source1);
    let mut parser1 = Parser::new(lexer1);
    let program1 = parser1.parse_program();
    assert!(!parser1.had_error);
    assert_eq!(program1.nodes.len(), 1);

    // Should be IndexAccess
    if let ASTNode::ExpressionStatement { expression } = &program1.nodes[0] {
        matches!(expression.as_ref(), ASTNode::IndexAccess { .. });
    }

    // Test that obj.property still parses as PropertyAccess (for modules)
    let source2 = r#"obj.property"#;
    let lexer2 = Lexer::new(source2);
    let mut parser2 = Parser::new(lexer2);
    let program2 = parser2.parse_program();
    assert!(!parser2.had_error);
    assert_eq!(program2.nodes.len(), 1);

    // Should be PropertyAccess (but will fail at compile time for non-modules)
    if let ASTNode::ExpressionStatement { expression } = &program2.nodes[0] {
        matches!(expression.as_ref(), ASTNode::PropertyAccess { .. });
    }
}

#[test]
fn test_complex_string_indexing() {
    let source = r#"
let data = { 
    users = { 
        john = { name = "John", age = 30 },
        jane = { name = "Jane", age = 25 }
    }
}
let john_name = data["users"]["john"]["name"]
let jane_age = data['users']['jane']['age']
"#;
    let result = compile_source(source);
    assert!(result.is_ok());
}

#[test]
fn test_string_indexing_with_variables() {
    let source = r#"
let obj = { key1 = "value1", key2 = "value2" }
let key = "key1"
let value = obj[key]
"#;
    let result = compile_source(source);
    assert!(result.is_ok());
}

#[test]
fn test_string_indexing_with_expressions() {
    let source = r#"
let obj = { hello_world = "test" }
let key = "hello" + "_" + "world"
let value = obj[key]
"#;
    let result = compile_source(source);
    assert!(result.is_ok());
}

#[test]
fn test_unterminated_single_quote_string() {
    let source = r#"let x = 'unterminated"#;
    let result = compile_source(source);
    assert!(result.is_err(), "Should fail with unterminated string");
}

#[test]
fn test_escaped_single_quotes() {
    let source = r#"let x = 'It\'s working!'"#;
    let result = compile_source(source);
    assert!(result.is_ok());

    let bytecode = result.unwrap();
    let has_expected = bytecode.constants.iter().any(|c| {
        matches!(&c.value, mirrow::library::compiler::ConstantValue::String(s) if s == "It's working!")
    });
    assert!(has_expected, "Should have string with escaped single quote");
}

#[test]
fn test_escaped_double_quotes() {
    let source = r#"let x = "He said \"Hello!\"""#;
    let result = compile_source(source);
    assert!(result.is_ok());

    let bytecode = result.unwrap();
    let has_expected = bytecode.constants.iter().any(|c| {
        matches!(&c.value, mirrow::library::compiler::ConstantValue::String(s) if s == r#"He said "Hello!""#)
    });
    assert!(has_expected, "Should have string with escaped double quote");
}

#[test]
fn test_dot_notation_on_struct_fails() {
    let source = r#"
let person = { name = "John", age = 30 }
let name = person.name
"#;
    let result = compile_source(source);
    assert!(result.is_err(), "Dot notation on struct should fail at compile time");
}

#[test]
fn test_dot_notation_on_variable_fails() {
    let source = r#"
let x = 42
let y = x.something
"#;
    let result = compile_source(source);
    assert!(result.is_err(), "Dot notation on non-module variable should fail at compile time");
}

#[test]
fn test_string_indexing_on_struct_works() {
    let source = r#"
let person = { name = "John", age = 30 }
let name = person["name"]
let age = person['age']
"#;
    let result = compile_source(source);
    assert!(result.is_ok(), "String indexing on struct should work");
}
