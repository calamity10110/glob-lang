# Language Structural Model

## Package Block System

The language uses an innovative **auto-organization** system that splits code into structured blocks.

### Primary Entry File

- **`.mn`** — Main file, human-friendly starting point
  - Can contain ANY block type
  - Compiler automatically splits into package blocks
  - User writes everything here initially

### Generated Package Block Files

The compiler auto-generates six structured file types:

1. **`imports.imp`** — All import statements
2. **`definitions.def`** — Type definitions, constants, schemas
3. **`async.asy`** — Async function definitions
4. **`functions.fnc`** — Synchronous function definitions
5. **`custom.cs`** — Foreign language blocks (Rust, C, Python, JS, TS, SQL)
6. **`main.mn`** — Cleaned entry point with only main logic

### Workflow

```
User writes:           Compiler generates:
main.mn        →       imports.imp
                       definitions.def
                       async.asy
                       functions.fnc
                       custom.cs
                       main.mn (cleaned)
```

## Secret Management Files

### Private Files (Never Published)

- **`project.scrt`**
  - Contains actual credentials
  - Always gitignored
  - Never included in published packages

- **`secret.def`**
  - Local decrypted environment secrets
  - Always gitignored
  - Runtime-only file

### Publishable File

- **`scrt.def`**
  - Contains only annotations and placeholders
  - Auto-generated if missing
  - Safe to publish
  - Cannot contain actual secrets (linter enforces)

### Secret Rules

1. Only `.scrt` and `secret.def` may contain real secrets
2. `scrt.def` is auto-generated with annotations only
3. Linter blocks secrets in `scrt.def`
4. Auto-redaction on publish
5. Auto-gitignore maintenance

## Package Structure

A complete package consists of:

```
my-package/
├── imports.imp          # Generated
├── definitions.def      # Generated
├── async.asy           # Generated
├── functions.fnc       # Generated
├── custom.cs           # Generated
├── main.mn             # Generated (cleaned)
├── scrt.def            # Generated (annotations only)
├── package.toml        # Package metadata
└── README.md           # Documentation
```

## Package Metadata (`package.toml`)

```toml
[package]
name = "my-package"
version = "1.0.0"
authors = ["Your Name <you@example.com>"]
description = "Package description"
license = "MIT"

[dependencies]
std = "1.0"
ui = "0.5"

[blocks]
imports = "imports.imp"
definitions = "definitions.def"
async = "async.asy"
functions = "functions.fnc"
custom = "custom.cs"
main = "main.mn"
secrets = "scrt.def"
```

## Publishing

When publishing a package:

1. Compiler validates all blocks
2. Secrets are auto-redacted
3. Only block files are included
4. `project.scrt` and `secret.def` are excluded
5. `scrt.def` with annotations is included
6. Package metadata is validated
7. Documentation is auto-generated

## Local Development

During development:

1. User edits `main.mn`
2. Compiler watches for changes
3. Auto-organizes into blocks on save
4. Linter provides real-time feedback
5. Secrets remain in `.scrt` files
6. Block files can be manually edited if needed

## Registry Structure

Packages in the registry follow this structure:

```
registry/
├── package-name/
│   ├── 1.0.0/
│   │   ├── imports.imp
│   │   ├── definitions.def
│   │   ├── async.asy
│   │   ├── functions.fnc
│   │   ├── custom.cs
│   │   ├── main.mn
│   │   ├── scrt.def
│   │   ├── package.toml
│   │   └── README.md
│   └── 1.0.1/
│       └── ...
```
