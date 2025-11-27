# TUI Design (Terminal User Interface)

## Overview

The TUI provides a minimalist ASCII-based interface for terminals and embedded displays.

## Supported UI Primitives

### Tree Display

```
def tree = ^÷^[tree]
ui.print(tree)
```

ASCII Output:
```
├── root
│   ├── branch1
│   │   ├── leaf1
│   │   └── leaf2
│   └── branch2
│       └── leaf3
```

### Slider

```
def slider = ^÷^[slider{min=0, max=100, value=50}]
ui.print(slider)
```

ASCII Output:
```
[====================|                    ] 50/100
```

### Button

```
def button = ^÷^[button{text="Click Me"}]
ui.print(button)
```

ASCII Output:
```
┌──────────┐
│ Click Me │
└──────────┘
```

### Text Block

```
def text = ^÷^[text{content="Hello, World!"}]
ui.print(text)
```

ASCII Output:
```
┌─────────────────┐
│ Hello, World!   │
└─────────────────┘
```

### Progress Bar

```
def progress = ^÷^[progress{value=75, max=100}]
ui.print(progress)
```

ASCII Output:
```
Progress: [███████████████████████████████████████████████████████████░░░░░░░░░░░░░░░] 75%
```

### Table

```
def table = ^÷^[table{
    headers=["Name", "Age", "City"],
    rows=[
        ["Alice", "30", "NYC"],
        ["Bob", "25", "LA"]
    ]
}]
ui.print(table)
```

ASCII Output:
```
┌───────┬─────┬──────┐
│ Name  │ Age │ City │
├───────┼─────┼──────┤
│ Alice │ 30  │ NYC  │
│ Bob   │ 25  │ LA   │
└───────┴─────┴──────┘
```

## Canvas (ASCII Art)

```
def canvas = ^÷^[canvas{width=40, height=10}]
canvas.draw_line(0, 0, 39, 9)
canvas.draw_rect(10, 2, 30, 8)
ui.print(canvas)
```

ASCII Output:
```
*                                       
 *                                      
  *       ┌──────────────────┐          
   *      │                  │          
    *     │                  │          
     *    │                  │          
      *   │                  │          
       *  │                  │          
        * └──────────────────┘          
         *                              
```

## Interactive Elements

### Input Field

```
def input = ^÷^[input{prompt="Enter name: "}]
name = ui.input(input)
print("Hello,", name)
```

Terminal Interaction:
```
Enter name: █
```

### Menu Selection

```
def menu = ^÷^[menu{
    title="Choose an option:",
    options=["Option 1", "Option 2", "Option 3"]
}]
choice = ui.select(menu)
print("Selected:", choice)
```

Terminal Output:
```
Choose an option:
  > Option 1
    Option 2
    Option 3
```

## Layout System

### Vertical Layout

```
def layout = ^÷^[vbox{
    children=[
        ^÷^[text{content="Header"}],
        ^÷^[slider{min=0, max=100}],
        ^÷^[button{text="Submit"}]
    ]
}]
ui.print(layout)
```

### Horizontal Layout

```
def layout = ^÷^[hbox{
    children=[
        ^÷^[text{content="Left"}],
        ^÷^[text{content="Center"}],
        ^÷^[text{content="Right"}]
    ]
}]
ui.print(layout)
```

## Color Support (ANSI)

```
def colored_text = ^÷^[text{
    content="Colorful!",
    fg="red",
    bg="black",
    bold=true
}]
ui.print(colored_text)
```

## Embedded Display Support

Works on:
- Serial terminals
- LCD character displays (16x2, 20x4)
- OLED displays
- E-ink displays
- Framebuffer consoles

### Example: LCD 16x2

```
imp embedded.lcd

def display = ^÷^[text{content="Hello World!"}]
lcd.init(16, 2)
lcd.print(display, row=0, col=0)
```

## TUI Framework Features

- **Responsive layouts** - Adapts to terminal size
- **Event handling** - Keyboard and mouse input
- **Scrolling** - For content larger than screen
- **Focus management** - Tab navigation between elements
- **Themes** - Customizable color schemes
- **Unicode support** - Box-drawing characters

## Example: Full TUI Application

```
imp ui.tui

def app = ^÷^[vbox{
    children=[
        ^÷^[text{content="System Monitor", bold=true}],
        ^÷^[progress{label="CPU", value=cpu_usage()}],
        ^÷^[progress{label="RAM", value=ram_usage()}],
        ^÷^[table{
            headers=["Process", "PID", "Memory"],
            rows=get_processes()
        }],
        ^÷^[hbox{
            children=[
                ^÷^[button{text="Refresh"}],
                ^÷^[button{text="Quit"}]
            ]
        }]
    ]
}]

mn main():
    tui.run(app)
```
