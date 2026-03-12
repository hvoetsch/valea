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

### Deterministic syntax
There should be **one obvious way** to express most ideas.

### Explicit semantics
No hidden allocations, no exceptions, no magic behavior.

### Machine-readable diagnostics
Compilers should talk to humans **and machines**.


### Canonical formatting
One style only.


### Small language surface
Fewer features → fewer edge cases → easier for agents.

---

## Why AI-native?

Autonomous software development requires languages that are:

- easy to generate
- easy to analyze
- easy to repair
- easy to verify

Valea enables workflows like:

AI agent receives goal
↓
agent writes Valea code
↓
compiler returns JSON diagnostics
↓
agent fixes code
↓
program runs


---

## Language Goals

Valea aims to be:

- statically typed
- compiled
- fast to build
- memory-safe by default
- simple ownership model
- no GC in the core
- explicit `Result` error handling
- small standard library

---

## Example

```valea
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() -> i32 {
    add(2, 3)
}

AI-Native Tooling

The Valea toolchain exposes structured outputs for agents.

Diagnostics
valea check main.val --json

Example output:

{
  "error": "type_mismatch",
  "expected": "i32",
  "found": "bool",
  "line": 12
}
AST
valea ast main.val --json
Formatter
valea fmt
Project Status

Valea is an early-stage experimental language.

The goal of the first milestone is simple:

Demonstrate that an AI agent can successfully generate, repair, and compile a Valea program autonomously.

Roadmap
Milestone 1 — AI-native MVP

parser

type checker

canonical formatter

JSON diagnostics

AST export

simple C backend

Milestone 2 — Core Language

variables

control flow

structs

modules

Result types

Milestone 3 — Ownership

move semantics

borrowing

deterministic destruction

Milestone 4 — Agent Tooling

structured lints

fix suggestions

capability metadata

reproducible builds

Getting Started

Build the compiler:

cd compiler
cargo build

Run:

valea check examples/hello.val
Contributing

Valea is designed as a community language experiment.

Good first areas:

parser improvements

formatter rules

JSON diagnostics

examples

documentation

See:

CONTRIBUTING.md
Vision

In the future, AI agents may:

design software

implement systems

repair bugs

optimize performance

deploy services

Valea explores the question:

What language would make that future easier?

License

MIT
