# GLOB Language - Complete Syntax Guide

**GLOB** (Global Language for Optimized Building) is a modern, multi-paradigm programming language designed for beginners and experts alike.

---

## Table of Contents

1. [Basic Concepts](#basic-concepts)
2. [Variables and Definitions](#variables-and-definitions)
3. [Functions](#functions)
4. [Imports and Packages](#imports-and-packages)
5. [Control Flow](#control-flow)
6. [Ownership Model](#ownership-model)
7. [UI Components](#ui-components)
8. [Multi-Language Integration](#multi-language-integration)
9. [Scientific Computing](#scientific-computing)
10. [Best Practices](#best-practices)

---

## Basic Concepts

### What is GLOB?

GLOB combines the best features of multiple languages:
- **Python's** readability and simplicity
- **Rust's** safety and ownership model
- **JavaScript's** async capabilities
- **SQL's** data-oriented expressions
- **Scientific notation** for math, physics, and chemistry

### Your First GLOB Program

```glob
# This is a comment - lines starting with # are ignored by the computer

# The main function is where your program starts
mn main():
    print("Hello, World!")  # Display text on screen
```

**Explanation:**
- `mn main():` - Defines the main entry point (where execution begins)
- `print()` - Built-in function to display text
- Indentation (spaces) defines code blocks, just like Python

---

## Variables and Definitions

### Simple Definitions

Use `def` to create variables (named storage for values):

```glob
# Basic types
def name = "Alice"           # String (text)
def age = 25                 # Integer (whole number)
def height = 5.8             # Float (decimal number)
def is_student = true        # Boolean (true/false)

# Lists (ordered collections)
def numbers = [1, 2, 3, 4, 5]
def names = ["Alice", "Bob", "Charlie"]

# Dictionaries (key-value pairs)
def person = {
    name: "Alice",
    age: 25,
    city: "New York"
}
```

### Type Annotations (Optional)

You can specify types explicitly for clarity:

```glob
def name: str = "Alice"      # String type
def age: int = 25            # Integer type
def price: float = 19.99     # Float type
def items: list = [1, 2, 3]  # List type
```

### Scientific Units

GLOB supports units for scientific computing:

```glob
# Physics
def speed = 10 m/s           # Meters per second
def acceleration = 9.81 m/s^2  # Meters per second squared
def mass = 75 kg             # Kilograms

# Chemistry
def pH = -log10([H+])        # pH calculation
def molarity = 2.5 mol/L     # Moles per liter

# Energy
def energy = m * c^2         # Einstein's equation
```

---

## Functions

### Synchronous Functions

Regular functions that execute immediately:

```glob
# Simple function
fn greet(name):
    return "Hello, " + name

# Function with multiple parameters
fn add(a, b):
    result = a + b
    return result

# Function with type annotations
fn multiply(x: int, y: int) -> int:
    return x * y

# Using the functions
message = greet("Alice")     # Returns "Hello, Alice"
sum = add(5, 3)              # Returns 8
product = multiply(4, 7)     # Returns 28
```

**Key Points:**
- `fn` keyword starts a function definition
- Parameters go in parentheses `()`
- Colon `:` starts the function body
- Indent the function body with spaces
- `return` sends a value back to the caller

### Async Functions

Functions that can wait for operations without blocking:

```glob
# Async function for network requests
asy fetch_data(url):
    # 'await' pauses until the request completes
    response = await http.get(url)
    return response.json()

# Async function with error handling
asy download_file(url, filename):
    try:
        data = await http.get(url)
        file.write(filename, data)
        return true
    catch error:
        print("Download failed:", error)
        return false

# Using async functions
mn main():
    # Must use 'await' when calling async functions
    data = await fetch_data("https://api.example.com/users")
    print(data)
```

**When to use async:**
- Network requests (HTTP, WebSocket)
- File I/O operations
- Database queries
- Any operation that might wait for external resources

---

## Imports and Packages

### Standard Library Imports

```glob
# Single import
imp std.io              # File and console I/O
imp std.http            # HTTP client
imp std.math            # Mathematical functions

# Multiple imports
imp std.io
imp std.http
imp std.json
imp ui                  # UI components
```

### Importing Python Packages

GLOB can use Python libraries directly:

```glob
# Import Python packages
imp python: numpy       # NumPy for numerical computing
imp python: pandas      # Pandas for data analysis
imp python: matplotlib  # Matplotlib for plotting

# Using Python packages
mn main():
    # Use numpy functions
    arr = numpy.array([1, 2, 3, 4, 5])
    mean = numpy.mean(arr)
    print("Mean:", mean)
```

### Importing JavaScript/Node Packages

```glob
# Import JavaScript/Node.js packages
imp js: express         # Express web framework
imp js: axios           # HTTP client
imp js: lodash          # Utility library

# Using JavaScript packages
asy start_server():
    app = express()
    app.get("/", fn(req, res):
        res.send("Hello from GLOB!")
    )
    await app.listen(3000)
```

### Importing Rust Crates

```glob
# Import Rust crates
imp rust: serde         # Serialization
imp rust: tokio         # Async runtime
imp rust: regex         # Regular expressions
```

### Custom Package Imports

```glob
# Import your own packages
imp myproject.utils     # From myproject/utils.mn
imp myproject.models    # From myproject/models.mn
imp myproject.api       # From myproject/api.mn
```

---

## Control Flow

### If-Elif-Else

```glob
# Simple if statement
if age >= 18:
    print("Adult")

# If-else
if temperature > 30:
    print("Hot day!")
else:
    print("Nice weather")

# If-elif-else chain
if score >= 90:
    grade = "A"
elif score >= 80:
    grade = "B"
elif score >= 70:
    grade = "C"
elif score >= 60:
    grade = "D"
else:
    grade = "F"
```

### Loops

```glob
# For loop - iterate over a collection
for number in [1, 2, 3, 4, 5]:
    print(number)

# For loop with range
for i in range(10):
    print("Count:", i)

# While loop - repeat while condition is true
count = 0
while count < 5:
    print(count)
    count = count + 1

# Infinite loop with break
loop:
    user_input = input("Enter 'quit' to exit: ")
    if user_input == "quit":
        break
    print("You entered:", user_input)

# Continue - skip to next iteration
for number in range(10):
    if number % 2 == 0:
        continue  # Skip even numbers
    print(number)  # Only prints odd numbers
```

---

## Ownership Model

GLOB uses Rust-inspired ownership for memory safety:

### Own - Transfer Ownership

```glob
# 'own' transfers ownership (moves the data)
fn process_data(own data):
    # This function now owns 'data'
    # Original caller can't use 'data' anymore
    result = transform(data)
    return result

# Usage
my_data = [1, 2, 3, 4, 5]
result = process_data(own my_data)
# my_data is no longer accessible here
```

### Ref - Borrow Reference

```glob
# 'ref' creates a reference (no copying, no moving)
fn calculate_sum(ref numbers):
    # Function can read 'numbers' but doesn't own it
    total = 0
    for num in numbers:
        total = total + num
    return total

# Usage
my_numbers = [1, 2, 3, 4, 5]
sum = calculate_sum(ref my_numbers)
# my_numbers is still accessible here
print(my_numbers)  # Works fine
```

### Copy - Explicit Duplication

```glob
# 'copy' creates a duplicate
fn modify_list(copy items):
    # Function gets a copy, original is unchanged
    items.append(99)
    return items

# Usage
original = [1, 2, 3]
modified = modify_list(copy original)
print(original)  # [1, 2, 3] - unchanged
print(modified)  # [1, 2, 3, 99] - has the new item
```

**Best Practice:**
- Use `ref` by default (most efficient)
- Use `own` when transferring large data
- Use `copy` only when you need a duplicate

---

## UI Components

### Inline UI Syntax

GLOB's unique feature: UI components as first-class syntax!

```glob
# The ^÷^ syntax creates UI components
def tree_widget = ^÷^[tree]
def slider_widget = ^÷^[slider{min=0, max=100, value=50}]
def button_widget = ^÷^[button{text="Click Me"}]

# Display UI components
ui.print(^÷^[tree])
ui.print(^÷^[slider{min=0, max=100, value=50}])
ui.print(^÷^[button{text="Submit", color="blue"}])
ui.print(^÷^[progress{value=75, max=100}])
```

### Available UI Components

```glob
# Text and Display
ui.print(^÷^[text{content="Hello", size=16, color="black"}])
ui.print(^÷^[heading{text="Title", level=1}])

# Input Components
ui.print(^÷^[input{placeholder="Enter name", type="text"}])
ui.print(^÷^[textarea{rows=5, cols=40}])
ui.print(^÷^[checkbox{label="Agree to terms", checked=false}])

# Interactive Components
ui.print(^÷^[button{text="Submit", onclick="handleSubmit"}])
ui.print(^÷^[slider{min=0, max=100, value=50, step=1}])
ui.print(^÷^[dropdown{options=["Option 1", "Option 2", "Option 3"]}])

# Data Display
ui.print(^÷^[table{headers=["Name", "Age"], rows=data}])
ui.print(^÷^[chart{type="bar", data=values}])
ui.print(^÷^[progress{value=75, max=100, label="Loading..."}])

# Layout Components
ui.print(^÷^[container{children=[widget1, widget2]}])
ui.print(^÷^[grid{columns=3, gap=10}])
```

---

## Multi-Language Integration

### Python Integration

```glob
# Embed Python code directly
cs python:
    import numpy as np
    import pandas as pd
    
    def analyze_data(data):
        df = pd.DataFrame(data)
        return df.describe()

# Use Python functions in GLOB
mn main():
    data = [[1, 2], [3, 4], [5, 6]]
    stats = analyze_data(data)
    print(stats)
```

### JavaScript Integration

```glob
# Embed JavaScript code
cs js:
    export function formatDate(date) {
        return new Date(date).toLocaleDateString();
    }
    
    export function fetchAPI(url) {
        return fetch(url).then(r => r.json());
    }

# Use JavaScript functions
mn main():
    formatted = formatDate("2024-01-15")
    print(formatted)
```

### Rust Integration

```glob
# Embed Rust code for performance
cs rust:
    fn fibonacci(n: u64) -> u64 {
        match n {
            0 => 0,
            1 => 1,
            _ => fibonacci(n - 1) + fibonacci(n - 2)
        }
    }

# Use Rust functions
mn main():
    result = fibonacci(10)
    print("Fibonacci(10) =", result)
```

### SQL Integration

```glob
# Embed SQL queries
cs sql:
    SELECT users.name, orders.total
    FROM users
    JOIN orders ON users.id = orders.user_id
    WHERE orders.total > 100
    ORDER BY orders.total DESC

# Execute SQL
mn main():
    results = db.execute(sql_query)
    for row in results:
        print(row.name, row.total)
```

---

## Scientific Computing

### Physics Calculations

```glob
# Define physical constants
def c = 299792458 m/s        # Speed of light
def G = 6.674e-11 N*m^2/kg^2  # Gravitational constant

# Physics formulas
fn kinetic_energy(mass, velocity):
    return 0.5 * mass * velocity^2

fn gravitational_force(m1, m2, distance):
    return G * m1 * m2 / distance^2

# Usage
energy = kinetic_energy(10 kg, 5 m/s)
force = gravitational_force(100 kg, 200 kg, 10 m)
```

### Chemistry Calculations

```glob
# Chemical calculations
fn calculate_pH(h_concentration):
    return -log10(h_concentration)

fn ideal_gas_law(pressure, volume, temperature):
    R = 8.314 J/(mol*K)  # Gas constant
    return (pressure * volume) / (R * temperature)

# Usage
ph_value = calculate_pH(1e-7)  # Neutral pH
moles = ideal_gas_law(101325 Pa, 0.001 m^3, 298 K)
```

### Mathematical Operations

```glob
# Import math functions
imp std.math

# Use mathematical functions
result = math.sin(math.PI / 2)  # Sine
root = math.sqrt(16)            # Square root
power = math.pow(2, 10)         # 2^10
log_val = math.log(100, 10)     # Log base 10
```

---

## Best Practices

### 1. Code Organization

```glob
# Good: Organize imports at the top
imp std.io
imp std.http
imp python: numpy

# Good: Group related definitions
def API_URL = "https://api.example.com"
def API_KEY = "your-key-here"
def TIMEOUT = 30

# Good: Clear function names
fn calculate_total_price(items, tax_rate):
    subtotal = sum(items)
    tax = subtotal * tax_rate
    return subtotal + tax
```

### 2. Error Handling

```glob
# Good: Handle errors explicitly
asy fetch_user_data(user_id):
    try:
        response = await http.get(f"/users/{user_id}")
        return response.json()
    catch error:
        print("Error fetching user:", error)
        return null

# Good: Validate inputs
fn divide(a, b):
    if b == 0:
        print("Error: Division by zero")
        return null
    return a / b
```

### 3. Use Ownership Wisely

```glob
# Good: Use 'ref' for read-only access
fn print_list(ref items):
    for item in items:
        print(item)

# Good: Use 'own' when transferring data
fn process_and_store(own data):
    processed = transform(data)
    database.save(processed)

# Good: Use 'copy' sparingly
fn backup_data(copy original):
    return original  # Returns a copy
```

### 4. Async Best Practices

```glob
# Good: Use async for I/O operations
asy load_user_profile(user_id):
    user = await db.get_user(user_id)
    posts = await db.get_posts(user_id)
    return {user: user, posts: posts}

# Good: Parallel async operations
asy load_dashboard():
    # Run multiple async operations in parallel
    users = await fetch_users()
    stats = await fetch_stats()
    notifications = await fetch_notifications()
    return {users, stats, notifications}
```

### 5. Comments and Documentation

```glob
# Good: Explain WHY, not WHAT
fn calculate_discount(price, customer_type):
    # Premium customers get 20% discount to encourage loyalty
    if customer_type == "premium":
        return price * 0.8
    return price

# Good: Document complex functions
#[
Function: process_payment
Purpose: Handles payment processing with retry logic
Parameters:
  - amount: Payment amount in cents
  - payment_method: Credit card or PayPal
Returns: Transaction ID or null on failure
]#
asy process_payment(amount, payment_method):
    # Implementation here
    pass
```

### 6. Naming Conventions

```glob
# Good: Use descriptive names
def user_count = 100          # Clear what it represents
def max_retry_attempts = 3    # Self-documenting

# Good: Use snake_case for variables and functions
fn calculate_total_price(items):
    pass

# Good: Use UPPER_CASE for constants
def MAX_CONNECTIONS = 100
def API_VERSION = "v2"
```

---

## Complete Example Program

```glob
# Complete GLOB program demonstrating key features

# Imports
imp std.io
imp std.http
imp python: pandas

# Constants
def API_URL = "https://api.example.com"
def MAX_RETRIES = 3

# Async function with error handling
asy fetch_data(endpoint):
    try:
        response = await http.get(API_URL + endpoint)
        return response.json()
    catch error:
        print("Fetch error:", error)
        return null

# Synchronous function with ownership
fn process_data(ref raw_data):
    # Process data without taking ownership
    processed = []
    for item in raw_data:
        if item.value > 0:
            processed.append(item)
    return processed

# Python integration for data analysis
cs python:
    import pandas as pd
    
    def analyze(data):
        df = pd.DataFrame(data)
        return df.describe().to_dict()

# Main entry point
mn main():
    print("Starting GLOB application...")
    
    # Fetch data asynchronously
    raw_data = await fetch_data("/data")
    
    if raw_data != null:
        # Process data
        processed = process_data(ref raw_data)
        
        # Analyze with Python
        stats = analyze(processed)
        
        # Display results with UI
        ui.print(^÷^[heading{text="Data Analysis Results"}])
        ui.print(^÷^[table{data=stats}])
        
        print("Analysis complete!")
    else:
        print("Failed to fetch data")
```

---

## Quick Reference

### Keywords
- `def` - Define variable
- `fn` - Define synchronous function
- `asy` - Define async function
- `mn` - Define main entry point
- `imp` - Import package
- `cs` - Custom language block
- `own`, `ref`, `copy` - Ownership keywords
- `await` - Wait for async operation
- `if`, `elif`, `else` - Conditionals
- `for`, `while`, `loop` - Loops
- `break`, `continue` - Loop control
- `return` - Return from function
- `try`, `catch` - Error handling

### Operators
- `+`, `-`, `*`, `/`, `%` - Arithmetic
- `^` - Power
- `==`, `!=`, `<`, `>`, `<=`, `>=` - Comparison
- `&&`, `||`, `!` - Logical
- `&`, `|`, `<<`, `>>` - Bitwise

### Built-in Functions
- `print()` - Display output
- `input()` - Get user input
- `len()` - Get length
- `range()` - Generate number sequence
- `sum()` - Sum of numbers
- `max()`, `min()` - Maximum/minimum

---

**Ready to start coding in GLOB? Check out the examples in the `examples/` directory!**
