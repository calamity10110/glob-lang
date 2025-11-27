# Syntax Overview

## Definitions

```
def speed = 10 m/s
def gravity = 9.81 m/s^2
def tree = ^÷^[tree]
def slider = ^÷^[slider]
def c = 299792458 m/s
def pH = -log10([H+])
```

## Ownership Model

Rust-inspired ownership with explicit keywords:

```
own item      # moves data, transfers ownership
ref value     # lightweight reference, no copy
copy obj      # explicit duplication
```

## Async Functions

```
asy fetch(url):
    res = await http.get(url)
    return res.text()

asy process_data(ref data):
    result = await compute(data)
    return result
```

## Synchronous Functions

```
fn add(a, b):
    return a + b

fn blink(own pin):
    gpio.high(pin)
    sleep(100ms)
    gpio.low(pin)
```

## UI Inline Syntax

Core innovation - UI elements as first-class syntax:

```
ui.print(^÷^[tree])
ui.print(^÷^[slider{min=0, max=100, value=50}])
ui.print(^÷^[button{text="Click Me"}])
ui.print(^÷^[image:icon])
ui.print(^÷^[sprite:player{x=10, y=20, width=32, height=32}])
```

## Foreign Language Blocks

### Rust

```
cs rust:
    fn sum(a: i32, b: i32) -> i32 { 
        a + b 
    }
```

### Python

```
cs python:
    import math
    def compute(x):
        return math.sin(x) + math.cos(x)
```

### JavaScript

```
cs js:
    export function greet(name) {
        return `Hello, ${name}!`;
    }
```

### TypeScript

```
cs ts:
    export function add(a: number, b: number): number {
        return a + b;
    }
```

### C

```
cs c:
    int fibonacci(int n) {
        return n <= 1 ? n : fibonacci(n-1) + fibonacci(n-2);
    }
```

### SQL

```
cs sql:
    select * from users where age > 18 order by name;
```

## Import System

```
imp std.io
imp std.http
imp ai.cv
imp ui
imp embedded.gpio
```

## Main Entry Point

```
mn main():
    print("Hello, World!")
    ui.print(^÷^[tree])
    data = await fetch("https://api.example.com")
    print(data)
```

## Scientific Notation Support

```
def F = m * a                    # Physics
def E = m * c^2                  # Energy
def v = d / t                    # Velocity
def pH = -log10([H+])           # Chemistry
def molarity = n / V             # Chemistry
```

## Control Flow

```
# Conditionals
if condition:
    do_something()
elif other_condition:
    do_other()
else:
    do_default()

# Loops
loop:
    # infinite loop
    break_if(condition)

for item in collection:
    process(item)

while condition:
    update()
```

## Comments

```
# Single line comment

#[
Multi-line
comment
block
]#
```

## Type Annotations (Optional)

```
def name: str = "Alice"
def age: int = 30
def speed: float = 10.5 m/s

fn calculate(x: int, y: int) -> int:
    return x + y
```
