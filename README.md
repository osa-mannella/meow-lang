# n

> A modern, functional programming language designed for clarity, performance, and developer experience.

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)

## ✨ Features

- **🚀 Functional First**: Immutable by default, with powerful pattern matching and automatic currying
- **⚡ High Performance**: Stack-based bytecode VM with efficient memory management and heap optimization
- **🔒 Memory Safe**: Built in Rust with automatic garbage collection and zero-cost abstractions
- **📦 Module System**: Clean import system with explicit effect tracking for side effects
- **🔄 Async Ready**: Built-in async/await with FramePackage-based task scheduling
- **🎯 Simple Syntax**: Clean, readable syntax that prioritizes developer experience

## 🚀 Quick Start

```n
// Hello World
func greet(name) {
    "Hello " + name
}

IO.print(greet("World"))

// Functional programming
let numbers = [1, 2, 3, 4, 5]
let doubled = map(numbers, fn(x) => x * 2)
let sum = reduce(doubled, fn(acc, x) => acc + x, 0)

IO.print(sum) // 30

// Pattern matching
let status = 200
match status {
    200 -> IO.print("Success!")
    404 -> IO.print("Not Found")
    _ -> IO.print("Unknown")
}
```

## 🏗️ Architecture

n is built with a modern, efficient architecture:

- **Two-pass Compiler**: Collects functions and constants, then generates optimized bytecode
- **Stack-based VM**: Fast execution with 2D stack frame system for efficient scoping
- **Heap Management**: Automatic placement of large objects with garbage collection
- **Native Interop**: Write low-level modules in Rust for maximum performance

## 📚 Language Features

### Immutable by Default

```n
let user = { name = "Alice", age = 30 }
let updatedUser = user <- { age = 31 }  // Creates new struct
```

### Automatic Currying

```n
func add3(a, b, c) { a + b + c }
let add1 = add3(1)           // Partially applied
let add1and2 = add1(2)       // Further applied
IO.print(add1and2(3))        // 6
```

### Powerful Pattern Matching

```n
let person = { name = "Bob", age = 25 }
match person {
    { name, age } -> IO.print($"Name: {name}, Age: {age}")
    _ -> IO.print("Unknown person")
}
```

### Effect Tracking

```n
import effect "IO"    // Explicit side effect declaration
import "math"         // Pure functions only
```

## 🛠️ Development Status

n is currently in active development. The core language features are implemented:

- ✅ Lexer and Parser
- ✅ Bytecode Compiler
- ✅ Stack-based VM
- ✅ Basic syntax and semantics
- 🔄 Heap management and GC
- 🔄 Module system
- 🔄 Async/await support

## 📖 Documentation

- [Language Specification](docs/SPEC.MD) - Complete language reference
- [Syntax Guide](docs/SYNTAX.md) - Language syntax and examples
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
