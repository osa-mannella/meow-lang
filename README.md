# n

> A modern, functional programming language designed for clarity, performance, and developer experience.

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)

## ✨ Currently Implemented Features

- **🚀 Functional Core**: Basic function definitions with automatic currying
- **⚡ High Performance**: Stack-based bytecode VM with efficient memory management
- **🔒 Memory Safe**: Built in Rust with heap management and garbage collection support
- **📦 Simple Scoping**: 2D stack frame system for efficient variable scoping
- **🎯 Clean Syntax**: Immutable variables and basic expressions

## 🚀 Quick Start

```n
// Basic variable declaration
let x = 5
let y = 10

// Simple function
func add(a, b) {
    a + b
}

// Function calls
let result = add(x, y)
```

## 🏗️ Architecture

n is built with a modern, efficient architecture:

- **Two-pass Compiler**: Collects functions and constants, then generates optimized bytecode
- **Stack-based VM**: Fast execution with 2D stack frame system for efficient scoping
- **Heap Management**: Automatic placement of large objects with garbage collection
- **Bytecode Optimization**: O(1) variable access through compile-time indexing

## 📚 Language Features

### Basic Syntax

```n
// Variables (immutable)
let counter = 0
let message = "Hello"

// Functions
func greet(name) {
    "Hello " + name
}

// Basic arithmetic
let sum = 5 + 3
let product = 4 * 6
```

### Function Calls

```n
// Direct function calls
let result = add(5, 3)

// Function composition
let doubled = multiply(2, 5)
```

## 🔮 Planned Features

The following features are planned but not yet implemented:

- **Pattern Matching**: Advanced structural pattern matching
- **Module System**: File-based imports with effect tracking
- **Async Support**: Built-in async/await with FramePackage scheduling
- **Collections**: Lists, maps, and functional collection operations
- **String Interpolation**: Template literal support
- **Structs**: Lightweight dynamic objects

## 🛠️ Development Status

n is currently in active development. The core language features are implemented:

- ✅ Lexer and Parser
- ✅ Bytecode Compiler
- ✅ Stack-based VM
- ✅ Basic syntax and semantics
- ✅ Heap management foundation
- 🔄 Advanced language features
- 🔄 Module system
- 🔄 Async/await support

## 📖 Documentation

- [Language Specification](docs/SPEC.MD) - Complete language reference and implementation details
- [Syntax Guide](docs/SYNTAX.md) - Planned language features and design goals
- [Bytecode Reference](docs/BYTECODE.md) - VM implementation details

## 🚧 Contributing

n is open for contributions! We welcome:

- Bug reports and feature requests
- Documentation improvements
- Performance optimizations
- Language feature proposals

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🤝 Acknowledgments

Built with ❤️ using [Rust](https://rust-lang.org) and inspired by modern functional programming languages.

---

**n** - Where simplicity meets performance.
