# WebUI Design (Canvas-Based IDE)

## Overview

The WebUI provides a modern, canvas-based visual editor called **Program Deck**. using Rust/Dioxus for TUI, App and WebUI.

## Features

### Node-Based Code Editor

- Visual representation of code blocks
- Drag-and-drop organization
- Real-time connections between nodes
- Automatic layout optimization

### Node Types

1. **Import Node** - Shows imported modules
2. **Definition Node** - Constants and type definitions
3. **Function Node** - Sync function blocks
4. **Async Node** - Async function blocks
5. **Custom Node** - Foreign language blocks
6. **UI Node** - Inline UI components
7. **Main Node** - Entry point

### Canvas Layout

```
┌─────────────────────────────────────────────────────────┐
│  Program Deck - main.mn                          [≡] [×] │
├─────────────────────────────────────────────────────────┤
│  File  Edit  View  Run  Debug  Help                     │
├──────────┬──────────────────────────────────────────────┤
│          │                                               │
│  Files   │         ┌──────────────┐                     │
│  ├─ src  │         │ Import Node  │                     │
│  │  └─ main.mn    │ std.io       │                     │
│  ├─ imports.imp   │ std.http     │                     │
│  ├─ definitions.def└──────────────┘                     │
│  ├─ async.asy          │                                │
│  ├─ functions.fnc      ▼                                │
│  └─ custom.cs     ┌──────────────┐                     │
│                    │ Async Node   │                     │
│  Outline          │ fetch()      │                     │
│  ├─ Imports       └──────────────┘                     │
│  ├─ Definitions        │                                │
│  ├─ Functions          ▼                                │
│  └─ Main          ┌──────────────┐                     │
│                    │ Main Node    │                     │
│                    │ main()       │                     │
│                    └──────────────┘                     │
│                                                          │
└──────────┴──────────────────────────────────────────────┘
```

## UI Component Rendering

### Live Preview

UI components defined with `^÷^[...]` syntax are rendered in real-time:

```
def slider = ^÷^[slider{min=0, max=100, value=50}]
```

Renders as interactive HTML5 slider in the preview pane.

### Preview Modes

1. **Auto-update** - Updates on every keystroke
2. **On-save** - Updates when file is saved
3. **Manual** - Updates on button click
4. **Tab-switch** - Updates when switching to preview tab

## Code Editor Features

### Syntax Highlighting

- Keywords in blue
- Strings in green
- Numbers in orange
- Comments in gray
- UI syntax in purple

### Auto-completion

- Import suggestions
- Function signatures
- Type hints
- UI component properties

### Inline Linting

- Real-time error detection
- Warning annotations
- Suggestions for improvements
- Quick-fix actions

### Code Actions

- Convert `fn` → `async`
- Extract to function
- Inline variable
- Organize imports
- Format code

## Visual Debugger

### Breakpoints

- Click line numbers to set breakpoints
- Conditional breakpoints
- Logpoints (non-breaking)

### Debug View

```
┌─────────────────────────────────────────┐
│ Variables                                │
├─────────────────────────────────────────┤
│ ├─ url: "https://api.example.com"       │
│ ├─ res: Response { status: 200 }        │
│ └─ data: { users: [...] }               │
├─────────────────────────────────────────┤
│ Call Stack                               │
├─────────────────────────────────────────┤
│ ├─ main() [main.mn:15]                  │
│ └─ fetch() [async.asy:5]                │
├─────────────────────────────────────────┤
│ Breakpoints                              │
├─────────────────────────────────────────┤
│ ✓ main.mn:15                            │
│ ✓ async.asy:8                           │
└─────────────────────────────────────────┘
```

## Performance Profiler

### Flame Graph

Visual representation of function call times.

### Metrics

- Function execution time
- Memory usage
- Async task count
- UI render time

## Package Explorer

### Visual Dependency Graph

```
     std.io
       │
       ▼
    my-app ──→ std.http
       │
       ▼
    ui.widgets
```

### Package Search

- Semantic search
- Popularity ranking
- Version compatibility
- Auto-install

## Live Collaboration

### Multi-user Editing

- Real-time cursor positions
- Collaborative debugging
- Shared terminal
- Chat integration

## Terminal Integration

### Embedded Terminal

- Run commands
- View logs
- Interactive REPL
- Build output

## UI Canvas Editor

### Visual UI Builder

Drag-and-drop UI components:

```
┌─────────────────────────────────────────┐
│ UI Canvas                                │
├─────────────────────────────────────────┤
│                                          │
│  ┌────────────────────────────────┐     │
│  │ Header                         │     │
│  └────────────────────────────────┘     │
│                                          │
│  [====================|         ] 50    │
│                                          │
│  ┌──────────┐  ┌──────────┐            │
│  │ Submit   │  │ Cancel   │            │
│  └──────────┘  └──────────┘            │
│                                          │
└─────────────────────────────────────────┘
```

Generates code:

```
def ui = ^÷^[vbox{
    children=[
        ^÷^[text{content="Header"}],
        ^÷^[slider{min=0, max=100, value=50}],
        ^÷^[hbox{
            children=[
                ^÷^[button{text="Submit"}],
                ^÷^[button{text="Cancel"}]
            ]
        }]
    ]
}]
```

## Responsive Design

- Adapts to screen size
- Mobile-friendly
- Touch support
- Keyboard shortcuts

## Themes

### Dark Mode

- Default theme
- Reduced eye strain
- Syntax highlighting optimized

### Light Mode

- High contrast
- Print-friendly

### Custom Themes

- User-defined color schemes
- Import/export themes

## Accessibility

- Screen reader support
- Keyboard navigation
- High contrast mode
- Adjustable font sizes

## Deployment

### Web-based (SaaS)

- No installation required
- Cloud storage
- Automatic updates

### Desktop App (Electron)

- Offline support
- Native performance
- Local file access

### Self-hosted

- Docker container
- On-premise deployment
- Custom authentication
