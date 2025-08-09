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
fn test_import_at_beginning_valid() {
    let source = r#"
import "IO"
let x = 42
"#;
    let result = compile_source(source);
    assert!(result.is_ok());
}

#[test]
fn test_import_after_statement_invalid() {
    let source = r#"
let x = 42
import "IO"
"#;
    let result = compile_source(source);
    assert!(result.is_err());
}

#[test]
fn test_import_after_function_invalid() {
    let source = r#"
func test() {
    42
}
import "IO"
"#;
    let result = compile_source(source);
    assert!(result.is_err());
}

#[test]
fn test_multiple_imports_at_beginning_valid() {
    let source = r#"
import "IO"
import "Math"
let x = 42
"#;
    let result = compile_source(source);
    assert!(result.is_ok());
}

#[test]
fn test_import_mixed_with_statements_invalid() {
    let source = r#"
import "IO"
let x = 42
import "Math"
let y = 43
"#;
    let result = compile_source(source);
    assert!(result.is_err());
}

#[test]
fn test_nonexistent_module_import() {
    let source = r#"
import "NonExistentModule"
let x = 42
"#;
    let result = compile_source(source);
    assert!(result.is_err());
}

#[test]
fn test_valid_builtin_module_import() {
    let source = r#"
import "IO"
let x = 42
"#;
    let result = compile_source(source);
    // This should succeed as IO is a built-in module
    assert!(result.is_ok());
}

#[test]
fn test_import_with_invalid_path_type() {
    // This tests the parser's validation that import requires a string literal
    let lexer = Lexer::new("import 123");
    let mut parser = Parser::new(lexer);
    let _program = parser.parse_program();
    
    // Parser should have encountered an error
    assert!(parser.had_error);
}

#[test]
fn test_empty_file_with_import() {
    let source = r#"import "IO""#;
    let result = compile_source(source);
    assert!(result.is_ok());
}

#[test]
fn test_import_after_if_statement_invalid() {
    let source = r#"
if true {
    let x = 42
}
import "IO"
"#;
    let result = compile_source(source);
    assert!(result.is_err());
}

#[test]
fn test_import_after_enum_declaration_invalid() {
    let source = r#"
enum TestEnum {
    Variant
}
import "IO"
"#;
    let result = compile_source(source);
    assert!(result.is_err());
}

#[test]
fn test_import_after_match_statement_invalid() {
    let source = r#"
match true {
    true -> 1
}
import "IO"
"#;
    let result = compile_source(source);
    assert!(result.is_err());
}

#[test]
fn test_import_ordering_complex_valid() {
    let source = r#"
import "IO"
import "Math"
import "String"

func main() {
    let x = 42
    if x > 0 {
        x + 1
    } else {
        x - 1
    }
}

enum Result {
    Success { value },
    Error { message }
}
"#;
    let result = compile_source(source);
    assert!(result.is_ok());
}