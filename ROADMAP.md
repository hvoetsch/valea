# Valea Roadmap

## Milestone 0 (this repository state)

- repository scaffold
- Rust MVP compiler
- deterministic parser/formatter
- JSON diagnostics
- JSON AST export
- C code emission for valid programs
- test coverage for lexer/parser/formatter/typeck/diagnostics

## Milestone 1

- spans propagated into parser/type checker precisely
- symbol table diagnostics with richer notes
- project manifest and deterministic package layout
- integrated `valea test` and `valea check` workflows

## Milestone 2

- ownership and deterministic destruction model (minimal version)
- explicit `Result` return conventions and sugar-free propagation
- tiny standard library bootstrap

## Milestone 3

- incremental compilation architecture
- stable IR export for analysis tools and autonomous repair agents
- reproducible build metadata and lockfile strategy
