# GUL v2.0 - Proposed Improved Syntax

**Version:** 2.0.0 (Proposed)  
**Date:** 2025-12-04 05:24:08 PST  
**Status:** Design Proposal  
**Goal:** Simplify, clarify, and modernize GUL syntax

---

## Design Principles

1. **Consistency** - One way to do things
2. **Clarity** - Obvious what code does
3. **Familiarity** - Learn from successful languages
4. **Simplicity** - Remove unnecessary complexity
5. **Explicitness** - No magic behavior

---

## Core Syntax

### Variables

```gul
// Immutable (default)
const name = "Alice"
const age = 25
const PI = 3.14159

// Mutable (explicit)
let mut count = 0
let mut total = 100

// With type annotations
const name: string = "Alice"
let mut count: int = 0

// Type inference
const items = [1, 2, 3]  // Inferred as Array<int>
```

**Rationale:**

- `const` for immutable (like JavaScript/Rust)
- `let mut` for mutable (like Rust)
- Clear, standard keywords
- Type inference by default

---

### Functions

```gul
// Basic function
fn greet(name: string) -> string {
    return "Hello, " + name
}

// With type inference
fn add(a, b) {
    return a + b  // Types inferred
}

// Async function
async fn fetch_data(url: string) -> Result<Data, Error> {
    const response = await http.get(url)
    return response.json()
}

// Generic function
fn first<T>(items: Array<T>) -> T {
    return items[0]
}
```

**Rationale:**

- Standard `fn` keyword
- `async` prefix for async functions
- Clear return types
- Generics support

---

### Types

```gul
// Primitive types
const age: int = 25
const price: float = 19.99
const name: string = "Alice"
const active: bool = true

// Collection types
const numbers: Array<int> = [1, 2, 3]
const person: Map<string, any> = {
    name: "Bob",
    age: 30
}
const unique: Set<int> = {1, 2, 3}

// Custom types
type User = {
    name: string,
    age: int,
    email: string
}

// Union types
type Result<T, E> = Ok(T) | Err(E)

// Enum types
enum Status {
    Pending,
    Active,
    Completed
}
```

**Rationale:**

- Standard type syntax
- Generics with `<T>`
- Union types for flexibility
- Enums for state

---

### Imports

```gul
// Single import
import std.io
import std.http

// Grouped imports
import {
    std.io,
    std.http,
    std.json
}

// Aliased imports
import std.io as io
import python.numpy as np

// From imports
import { get, post } from std.http
import { User, Post } from models

// External language imports
import python.numpy
import rust.tokio
import js.express
```

**Rationale:**

- Standard `import` keyword
- One syntax for grouping: `{}`
- Clear aliasing with `as`
- Explicit external imports

---

### Control Flow

```gul
// If-else
if age >= 18 {
    print("Adult")
} else if age >= 13 {
    print("Teenager")
} else {
    print("Child")
}

// For loop
for item in items {
    print(item)
}

// While loop
while count < 10 {
    print(count)
    count = count + 1
}

// Loop with break/continue
loop {
    const input = read_input()
    if input == "quit" {
        break
    }
    if input == "skip" {
        continue
    }
    process(input)
}
```

**Rationale:**

- Standard control flow
- Braces for blocks
- Clear keywords

---

### Pattern Matching

```gul
// Match expression
match value {
    Ok(data) => process(data),
    Err(error) => handle_error(error),
}

// Match with guards
match age {
    x if x < 13 => "Child",
    x if x < 18 => "Teenager",
    _ => "Adult"
}

// Destructuring
match user {
    { name, age } => print(name, age),
}

// Multiple patterns
match status {
    Pending | Processing => "In progress",
    Completed => "Done",
    Failed => "Error"
}
```

**Rationale:**

- Essential for modern language
- Clear syntax
- Powerful destructuring

---

### Error Handling

```gul
// Result type
fn divide(a: int, b: int) -> Result<int, string> {
    if b == 0 {
        return Err("Division by zero")
    }
    return Ok(a / b)
}

// Using results
const result = divide(10, 2)
match result {
    Ok(value) => print("Result:", value),
    Err(error) => print("Error:", error),
}

// Option type
fn find_user(id: int) -> Option<User> {
    if user_exists(id) {
        return Some(get_user(id))
    }
    return None
}

// Try operator
fn process() -> Result<Data, Error> {
    const data = fetch_data()?  // Propagates error
    const processed = transform(data)?
    return Ok(processed)
}
```

**Rationale:**

- Result/Option types (like Rust)
- Pattern matching for handling
- `?` operator for propagation

---

### Multi-Language Integration

```gul
// Python integration
extern python {
    fn analyze(data: Array<float>) -> float {
        import numpy as np
        return np.mean(data)
    }

    fn plot(data: Array<float>) {
        import matplotlib.pyplot as plt
        plt.plot(data)
        plt.show()
    }
}

// Rust integration
extern rust {
    fn fast_compute(n: u64) -> u64 {
        n * n
    }
}

// JavaScript integration
extern js {
    fn format_date(date: string) -> string {
        return new Date(date).toLocaleDateString()
    }
}

// SQL integration
extern sql {
    query get_users() -> Array<User> {
        SELECT * FROM users WHERE active = true
    }
}

// Usage
const stats = python.analyze(data)
const result = rust.fast_compute(100)
const formatted = js.format_date("2024-01-15")
const users = sql.get_users()
```

**Rationale:**

- Clear `extern` keyword
- Explicit language blocks
- Type-safe boundaries
- Clear function signatures

---

### Async/Await

```gul
// Async function
async fn fetch_users() -> Result<Array<User>, Error> {
    const response = await http.get("/users")
    return response.json()
}

// Parallel execution
async fn fetch_all() {
    const [users, posts, comments] = await Promise.all([
        fetch_users(),
        fetch_posts(),
        fetch_comments()
    ])
    return { users, posts, comments }
}

// Async iteration
async fn process_stream(stream) {
    for await item in stream {
        await process(item)
    }
}
```

**Rationale:**

- Standard async/await
- Clear syntax
- Parallel execution support

---

### Ownership (Optional)

```gul
// By default: automatic memory management
fn process(data: Array<int>) {
    // data is borrowed
    print(data)
}

// Explicit ownership when needed
fn consume(data: own Array<int>) {
    // Takes ownership
    transform(data)
    // data is dropped here
}

// Explicit borrowing
fn read(data: ref Array<int>) {
    // Borrows immutably
    print(data)
}

// Mutable borrowing
fn modify(data: mut ref Array<int>) {
    // Borrows mutably
    data.push(42)
}
```

**Rationale:**

- Automatic by default
- Explicit when needed
- Clear keywords

---

### Standard Library

```gul
// File I/O
import std.io

const content = io.read_file("data.txt")
io.write_file("output.txt", content)

// HTTP
import std.http

const response = await http.get("https://api.example.com")
const data = response.json()

// JSON
import std.json

const obj = json.parse('{"name": "Alice"}')
const str = json.stringify(obj)

// Math
import std.math

const sqrt = math.sqrt(16)
const sin = math.sin(math.PI / 2)

// Time
import std.time

const now = time.now()
const formatted = time.format(now, "YYYY-MM-DD")

// Testing
import std.test

test "addition works" {
    assert(1 + 1 == 2)
}
```

**Rationale:**

- Comprehensive standard library
- Clear module names
- Consistent API

---

## Complete Example

```gul
// Complete GUL v2.0 program

// Imports
import std.io
import std.http
import {
    python.numpy as np,
    python.pandas as pd
}

// Types
type User = {
    id: int,
    name: string,
    email: string,
    age: int
}

type ApiResponse<T> = {
    data: T,
    status: int,
    message: string
}

// Constants
const API_URL = "https://api.example.com"
const MAX_RETRIES = 3

// Functions
async fn fetch_users() -> Result<Array<User>, Error> {
    let mut retries = 0

    loop {
        const response = await http.get(API_URL + "/users")

        match response.status {
            200 => {
                const data: ApiResponse<Array<User>> = response.json()
                return Ok(data.data)
            },
            _ if retries < MAX_RETRIES => {
                retries = retries + 1
                await sleep(1000)
            },
            _ => {
                return Err("Failed to fetch users")
            }
        }
    }
}

fn analyze_users(users: Array<User>) -> Map<string, float> {
    const ages = users.map(u => u.age)

    return {
        count: users.length,
        average_age: python.np.mean(ages),
        median_age: python.np.median(ages)
    }
}

// Python integration
extern python {
    fn create_dataframe(users: Array<User>) -> DataFrame {
        import pandas as pd
        return pd.DataFrame(users)
    }

    fn generate_report(df: DataFrame) -> string {
        return df.describe().to_string()
    }
}

// Main
async fn main() {
    print("Fetching users...")

    const result = await fetch_users()

    match result {
        Ok(users) => {
            print("Found", users.length, "users")

            const stats = analyze_users(users)
            print("Statistics:", stats)

            const df = python.create_dataframe(users)
            const report = python.generate_report(df)
            print(report)

            io.write_file("report.txt", report)
            print("Report saved!")
        },
        Err(error) => {
            print("Error:", error)
        }
    }
}
```

---

## Migration from v1.0

### Syntax Changes

| v1.0          | v2.0               | Reason                    |
| ------------- | ------------------ | ------------------------- |
| `def x = 5`   | `const x = 5`      | Standard keyword          |
| `?x = 5`      | `let mut x = 5`    | Clear mutability          |
| `@int x`      | `x: int`           | Standard type syntax      |
| `mn main()`   | `fn main()`        | Consistent with functions |
| `asy fn()`    | `async fn()`       | Standard async            |
| `imp pkg`     | `import pkg`       | Standard keyword          |
| `@cs python:` | `extern python {}` | Clear boundaries          |
| `^÷^[ui]`     | `ui.render()`      | Standard syntax           |

### Migration Tool

```bash
# Automatic migration
gul migrate v1-to-v2 main.mn

# Manual migration guide
gul migrate --guide
```

---

## Benefits of v2.0

1. **Easier to Learn** - Familiar syntax
2. **Better Tooling** - Standard patterns
3. **Clearer Code** - Explicit intent
4. **Less Confusion** - One way to do things
5. **Better Errors** - Clear error messages

---

## Conclusion

GUL v2.0 simplifies the language while keeping its unique features:

**Kept:**

- Multi-language integration
- Async-first design
- Scientific computing support
- Secret management

**Improved:**

- Consistent syntax
- Standard keywords
- Clear type system
- Better error handling
- Simplified imports

**Removed:**

- `?` prefix for mutability
- Multiple import syntaxes
- `@` annotation overload
- `mn` keyword
- `^÷^` UI syntax
- Automatic block splitting

**Result:** A modern, clean, powerful language that's easy to learn and use.

---

**Proposed:** 2025-12-04 05:24:08 PST  
**Status:** Design Proposal  
**Next Steps:** Community feedback, prototype implementation
