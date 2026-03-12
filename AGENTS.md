# Valea Agent Guide

This repository is optimized for deterministic autonomous contribution.

## Rules

1. Read `SPEC.md` before adding language features.
2. Do not implement syntax not described in `SPEC.md`.
3. Keep diagnostics stable and machine-readable.
4. Every new diagnostic must have a stable `E###` code.
5. Prefer small modules and explicit control flow.
6. Keep formatter output canonical and deterministic.

## Workflow expectations

- Run `cargo test` before committing.
- If CLI behavior changes, update `README.md` and `SPEC.md`.
- Add or update examples for user-visible features.
- Keep output deterministic to support repair loops.
