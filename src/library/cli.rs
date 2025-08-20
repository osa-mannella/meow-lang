use std::io::{self, Write};

pub fn print_help() {
    println!("Mirrow - The Reflective Language");
    println!();
    println!("USAGE:");
    println!("    mirrow [OPTIONS] [FILE]");
    println!();
    println!("ARGS:");
    println!("    <FILE>    The .n file to execute");
    println!();
    println!("OPTIONS:");
    println!("    -h, --help       Print help information");
    println!("    -V, --version    Print version information");
    println!("    --debug          Enable debug output");
    println!();
    println!("EXAMPLES:");
    println!("    mirrow main.n                # Run main.n");
    println!("    mirrow --debug example.n      # Run with debug output");
}

pub fn print_version() {
    println!("mirrow {}", env!("CARGO_PKG_VERSION"));
    println!("🌊 The Reflective Language");
}

pub fn print_error(message: &str) {
    eprintln!("❌ Error: {}", message);
}

pub fn print_success(message: &str) {
    println!("✅ {}", message);
}
