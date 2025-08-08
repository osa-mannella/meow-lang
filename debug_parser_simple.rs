use mirrow::library::lexer::*;
use mirrow::library::parser::*;

fn main() {
    let source = "if true { 42 }";
    println!("Parsing: {}", source);
    
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);
    
    println!("About to call parse_program()");
    let program = parser.parse_program();
    
    if parser.had_error {
        println!("Parser had errors");
    } else if program.nodes.is_empty() {
        println!("No nodes parsed");
    } else {
        println!("Successfully parsed {} nodes", program.nodes.len());
        println!("First node: {:?}", program.nodes[0]);
    }
}