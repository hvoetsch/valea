# CLAUDE.md — Valea Compiler

This file tells Claude Code how to develop in this repo autonomously.

## What is Valea?

An AI-native systems language. The compiler is written in Rust and targets C as output.
The language is intentionally minimal so AI agents can generate, validate, and repair
programs without ambiguity.

## Build & Test

```bash
cargo build --release          # build the compiler
cargo test                     # run all tests
./target/release/valea --help  # quick smoke test
```

## Compiler CLI

```bash
# Check a .va file (human output)
./target/release/valea check examples/fibonacci.va

# Check with machine-readable JSON (for agents)
./target/release/valea check examples/fibonacci.va --json

# Dump AST as JSON
./target/release/valea ast examples/fibonacci.va --json

# Canonically format a file (rewrites in-place)
./target/release/valea fmt examples/fibonacci.va

# Check formatting without writing (exits 1 if not formatted)
./target/release/valea fmt examples/fibonacci.va --check

# Emit C code
./target/release/valea emit-c examples/fibonacci.va
```

## The Valea Language (MVP Spec)

**Only these constructs exist.** Do not invent syntax.

```
fn name() -> type {
    expression
}
```

- **Types:** `int` (i64), `bool`
- **Expressions:** integer literals, `true`, `false`, addition (`+`), zero-arg function calls
- **No:** parameters, variables, control flow, loops, recursion, imports

### Valid example

```
fn base() -> int {
    100
}

fn total() -> int {
    base() + 25
}

fn is_ready() -> bool {
    true
}
```

### Canonical format (enforced by `fmt`)

- 4-space indent inside function body
- One blank line between functions
- `fn name() -> type {\n    expr\n}`

## Error Code Reference

| Code | Phase  | Meaning                                      | Fix                                    |
|------|--------|----------------------------------------------|----------------------------------------|
| E001 | Lex    | Lone `-` (did you mean `->`?)                | Replace with `->` or remove            |
| E002 | Lex    | Integer literal out of i64 range             | Use a smaller number                   |
| E003 | Lex    | Unexpected character                         | Remove the invalid character           |
| E100 | Parse  | Expected `fn`                                | Add `fn` keyword                       |
| E101 | Parse  | Expected `(` after function name             | Add `(`                                |
| E102 | Parse  | Expected `)` (parameters not supported)      | Remove parameters, use `()`           |
| E103 | Parse  | Expected `->` before return type             | Add `->`                               |
| E104 | Parse  | Expected `{` to start body                   | Add `{`                                |
| E105 | Parse  | Expected `}` to end body                     | Add `}`                                |
| E106 | Parse  | Variable references not supported            | Call a function instead: `name()`     |
| E107 | Parse  | Expected `)` after function call             | Remove arguments, use `name()`        |
| E108 | Parse  | Expected an expression                       | Add a valid expression                 |
| E109 | Parse  | Expected an identifier                       | Add a function name                    |
| E110 | Parse  | Expected return type `int` or `bool`         | Use `int` or `bool`                    |
| E200 | Type   | Duplicate function name                      | Rename one of the functions            |
| E201 | Type   | Return type mismatch                         | Fix body or change declared type       |
| E202 | Type   | Unknown function called                      | Define the missing function            |
| E203 | Type   | `+` used with non-int operands               | Replace bool operand with int          |

## Development Workflow

1. **Before adding features** — Read `SPEC.md`. Do not implement syntax not in the spec.
2. **Write tests first** — Add cases to `tests/mvp.rs` before implementing.
3. **Run tests** — `cargo test` must pass before committing.
4. **Format examples** — All `.va` files in `examples/` must pass `fmt --check`.
5. **Stable error codes** — Never change the meaning of an existing E### code.

## Module Map

| File | Purpose |
|------|---------|
| `src/lexer.rs` | Tokenizer — byte offsets, E001–E003 |
| `src/parser.rs` | Recursive descent — E100–E110 |
| `src/ast.rs` | AST types (Program, FunctionDecl, Expr, Type) |
| `src/typeck.rs` | Two-pass type checker — E200–E299 |
| `src/codegen.rs` | Tree-walk C emitter with forward declarations |
| `src/formatter.rs` | Canonical formatter |
| `src/diagnostics.rs` | Span + Diagnostic + line:col helpers |
| `src/json.rs` | Manual JSON serialization for agent output |
| `src/main.rs` | CLI entry point (check, ast, fmt, emit-c) |
| `src/lib.rs` | Public API (parse_source, check_source) |
| `tests/mvp.rs` | Integration tests |

## Agent Demo

```bash
pip install anthropic
ANTHROPIC_API_KEY=sk-... python examples/claude_programs_valea.py
```

See `examples/agent_loop.py` for a simpler demo without the Claude API.
