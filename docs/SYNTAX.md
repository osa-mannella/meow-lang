# Variables

## Declaration

Variables in Mirrow are declared using the `let` keyword.  
They are dynamically typed by default, and values can be reassigned.

```mirrow
let x = 5
x = x + 1
print(x) // 6
```

---

## Constants

To declare a variable that **cannot be reassigned**, use `const let`.  
The value reference cannot change at runtime, but if it is a mutable object (like an array or JSON object), its contents may still be modified.

```mirrow
const let settings = {
    theme = "dark",
    version = 1
}

settings.theme = "light" // allowed (object is mutable)
settings = {} // error (cannot reassign)
```

---

## Naming Rules

- Variable names may include letters, numbers, and underscores, but cannot begin with a number.
- Mirrow is case-sensitive: `value` and `Value` are considered different identifiers.

---

## Best Practices

- Use `const let` for values that should never be reassigned.
- Use plain `let` when the value is expected to change.
- Prefer descriptive names:
  ```mirrow
  let userName = "Alice"
  const let maxUsers = 100
  ```

---

## Future Considerations

- **Optional type hints** (planned feature):
  ```mirrow
  let count: number = 5
  const let config: object = {}
  ```
- **Immutable deep structures** (planned feature) for true runtime immutability.

# Functions

## Declaration

Functions are declared using the `func` keyword, followed by a name, optional parameter list, and a block body:

```mirrow
func greet(name) {
    print("Hello " + name)
}
```

### Rules

- **Parameters**: Declared inside parentheses, separated by commas.  
  Example:
  ```mirrow
  func add(a, b) {
    return a + b
  }
  ```
- **Return Types**: Functions return `null` by default if no `return` value is provided.  
  Example:
  ```mirrow
  func hello() {
    print("Hello!")
  }
  // returns null
  ```
- **Future Type Hints** _(planned feature)_:
  ```mirrow
  func greet(name: string): string {
    return "Hello " + name
  }
  ```
- **Default Parameter Values** _(planned feature)_:
  ```mirrow
  func greet(name = "Guest") {
    print("Hello " + name)
  }
  ```

---

## Calling Functions

Functions are called by name, followed by parentheses containing arguments:

```mirrow
let sum = add(1, 2)
greet("Alice")
```

---

## Return Values

Use the `return` keyword to explicitly return a value:

```mirrow
func multiply(a, b) {
    return a * b
}

let result = multiply(3, 4)
print(result) // 12
```

- If no `return` statement is present, the function implicitly returns `null`.

---

## Anonymous Functions (Lambdas)

Mirrow supports inline anonymous functions using arrow syntax:

```mirrow
let adder = (a, b) => a + b
print(adder(5, 10)) // 15
```

- Lambdas implicitly return the last expression.
- Parentheses may be omitted for single-parameter lambdas:

```mirrow
let square = (x) -> x * x
```

---

## Reflection

Functions are **first-class citizens** and support native reflection via the `#` operator:

```mirrow
func greet(name) {
    print("Hello " + name)
}

let meta = #greet
print(meta.name) // "greet"
print(meta.params) // ["name"]
print(meta.arity) // 1
```

### Metadata Object Fields

- **name** → `string` (function name)
- **params** → `list<string>` (parameter names)
- **arity** → `number` (parameter count)
- **location** _(future)_ → source file & line number
- **docstring** _(future)_ → documentation string

---

## Future Considerations

- **Optional type hints** for parameters and return values.
- **Docstring reflection** to access function documentation at runtime.
- **Function decorators/macros** using `#` operators for compile-time transformations.
- **First-class async functions** and event-based callbacks.

# Loops

## While Loop

The `while` loop executes a block of code as long as its condition evaluates to true.

```mirrow
let count = 0
while count < 5 {
    print(count)
    count = count + 1
}
```

---

## For-In Loop

The `for` loop is used for iterating over ranges and iterable collections.

### Range iteration:

```mirrow
for i in range(0, 5) {
    print(i)
}
```

### String iteration:

```mirrow
for char in "hello" {
    print(char)
}
```

---

## Break and Continue

- **break** → immediately exits the loop.
- **continue** → skips the current iteration and continues with the next.

```mirrow
for i in range(0, 10) {
    if i == 5 {
        break
    }
    if i % 2 == 0 {
        continue
    }
    print(i)
}
```

---

## Future Considerations

- **Iterable Objects**:  
  Any object implementing iterable metadata will be supported:
  ```mirrow
  for param in #greet.params {
      print(param)
  }
  ```
- **Async Iteration** (future):  
  Support for asynchronous iteration over data sources:
  ```mirrow
  for await line in file {
      print(line)
  }
  ```

# Types

## Overview

Mirrow is dynamically typed in v1.  
Variables can hold values of any type, and functions may return any type unless specified otherwise in future versions.

---

## Primitive Types

### Number

Represents integers and floating-point values:

```mirrow
let x = 42
let pi = 3.14159
```

### String

A sequence of characters, enclosed in double quotes:

```mirrow
let name = "Alice"
let greeting = "Hello, " + name
```

### Boolean

Logical true/false values:

```mirrow
let isReady = true
let isDone = false
```

### Null

Represents the absence of a value:

```mirrow
let data = null
```

---

## Composite Types

### List

Ordered, index-based collections:

```mirrow
let numbers = [1, 2, 3, 4]
print(numbers[0]) // 1
```

### Object (Map / JSON-like)

Key-value pairs, similar to JSON:

```mirrow
let user = {
    name = "Alice",
    age = 30
}
print(user.name) // "Alice"
```

---

## Reflection of Types

All values have type metadata accessible via the `#` operator:

```mirrow
let name = "Alice"
let meta = #name
print(meta.type) // "string"
```

### Metadata Object Fields

- **type** → The runtime type name as a string.
- **fields** (for objects) → List of keys.
- **length** (for lists/strings) → Length of the collection.

---

## Future Considerations

- **Optional type annotations**:
  ```mirrow
  let count: number = 5
  const let config: object = {}
  ```
- **Custom types & classes**.
- **Generics and static type checking** as an opt-in feature.

# Conditionals

## If / Else If / Else

Conditional execution uses `if`, optional `else if`, and `else` blocks.

```mirrow
let x = 10

if x > 0 {
    print("Positive")
} else if x == 0 {
    print("Zero")
} else {
    print("Negative")
}
```

- Parentheses around conditions are optional.
- Blocks are required for multi-line branches.

---

# Operators

## Arithmetic

- Addition: `a + b`
- Subtraction: `a - b`
- Multiplication: `a * b`
- Division: `a / b`
- Modulus: `a % b`

## Comparison

- Equal: `a == b`
- Not equal: `a != b`
- Greater than: `a > b`
- Less than: `a < b`
- Greater than or equal: `a >= b`
- Less than or equal: `a <= b`

## Logical

- And: `a && b`
- Or: `a || b`
- Not: `!a`

## Assignment

- Standard: `=`
- Compound: `+=`, `-=`, `*=`, `/=`, `%=` (syntactic sugar for operations with assignment)

## String Operations

- Concatenation: `+` (works on strings as well)
  ```mirrow
  let greeting = "Hello, " + "World"
  ```

---

# Imports

## Module Import

Modules can be imported using the `import` keyword.
Paths may be relative or absolute.

```mirrow
import "math"
import "./utils"
```

- Imported modules expose their public functions and constants.
- Future versions may allow selective imports:
  ```mirrow
  import { add, subtract } from "math"
  ```

## Future Considerations

- **Remote modules**: loading code from URLs.
- **Package manager integration**.

---

# Error Handling

## Throwing Errors

Use the `throw` keyword to raise an exception.

```mirrow
throw "Something went wrong"
```

## Try / Catch

Handle errors using `try` and `catch`:

```mirrow
try {
    riskyOperation()
} catch (err) {
    print("Caught error: " + err)
}
```

- `err` is a runtime value representing the thrown error.
- If no `catch` block is provided, the error propagates up the call stack.

## Future Considerations

- **Custom error types** with metadata.
- **Finally blocks** for guaranteed cleanup:
  ```mirrow
  try {
    openFile()
  } catch (err) {
    print(err)
  } finally {
    closeFile()
  }
  ```
