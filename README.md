# Valea

**Valea is an AI-native systems programming language.**

Programming languages were built for humans.
Valea is built for **humans and autonomous AI agents**.

---

## The Idea

AI agents are increasingly capable of writing software.

But today's programming languages were **not designed for autonomous code generation, repair, and verification**.

Valea explores a simple idea:

> What would a programming language look like if it were designed for both humans **and AI agents** from the start?

---

## Core Principles

Valea focuses on five properties:

- **Deterministic syntax**: one obvious way to express most ideas.
- **Explicit semantics**: no hidden allocations, no exceptions, no magic behavior.
- **Machine-readable diagnostics**: compiler output should support both humans and tools.
- **Canonical formatting**: one stable style.
- **Small language surface**: fewer edge cases and easier automated repair loops.

---

## Why AI-native?

Autonomous software development requires languages that are:

- easy to generate
- easy to analyze
- easy to repair
- easy to verify

Typical Valea workflow:

1. Agent receives a goal.
2. Agent writes Valea code.
3. Compiler returns structured diagnostics.
4. Agent applies fixes.
5. Program compiles and runs.

---

## Example

```valea
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() -> i32 {
    add(2, 3)
}
```

---

## AI-native Tooling

### Diagnostics

```sh
valea check examples/type_error.va --json
```

Example output:

```json
[
  {
    "code": "E001",
    "message": "type mismatch",
    "line": 2,
    "col": 5
  }
]
```

### AST export

```sh
valea ast examples/ok.va --json
```

### Formatter

```sh
valea fmt examples/ok.va
```

### C emission

```sh
valea emit-c examples/ok.va
```

---

## Project Status

Valea is an early-stage experimental language.

Current milestone highlights:

- Rust MVP compiler
- deterministic lexer/parser/formatter
- JSON diagnostics
- JSON AST export
- simple C backend

See [ROADMAP.md](ROADMAP.md) for planned milestones.

---

## Getting Started

Build the compiler:

```sh
cargo build
```

Run checks:

```sh
cargo test
cargo run -- check examples/ok.va
```

---

## Contributing

Valea is designed as a community language experiment.

Good first contribution areas:

- parser improvements
- formatter rules
- JSON diagnostics
- examples
- documentation

Language and toolchain behavior are specified in [SPEC.md](SPEC.md).

---
## Demo

https://asciinema.org/a/834560

## License

MIT
