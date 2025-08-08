use std::path::Path;

fn main() {
    let source = "if true { 42 }";
    println!("Parsing: {}", source);
    
    let lexer = mirrow::library::lexer::Lexer::new(source);
    let mut parser = mirrow::library::parser::Parser::new(lexer);
    
    if let Some(result) = parser.parse_expression(0) {
        println!("Success: {:?}", result);
    } else {
        println!("Failed to parse");
    }
    
    if parser.had_error {
        println!("Parser had errors");
    }
}