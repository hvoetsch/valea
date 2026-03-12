# Valea

Valea is an experimental AI-native systems programming language.

This repository contains the **first MVP compiler** written in Rust. The MVP demonstrates a tight autonomous loop:

1. Write Valea code.
2. Run `valea check --json` for machine-readable diagnostics.
3. Repair code deterministically.
4. Run `valea emit-c` on valid programs.

## Current commands

- `cargo run -- check <file.va>`
- `cargo run -- check <file.va> --json`
- `cargo run -- ast <file.va> --json`
- `cargo run -- fmt <file.va>`
- `cargo run -- emit-c <file.va>`

## Language subset (MVP)

- function declarations (`fn name() -> type { expr }`)
- integer literals
- boolean literals
- zero-argument function calls
- binary addition (`+`)
- explicit return types (`int`, `bool`)

See `SPEC.md` for the exact grammar and constraints.
