# Valea MVP Specification

## 1. Scope

This document defines the allowed surface area for the initial Valea compiler.
Anything not listed here is out of scope and must not be implemented without updating this spec.

## 2. Source files

- UTF-8 text
- `.va` extension by convention
- deterministic formatting produced by `valea fmt`

## 3. Grammar (MVP)

```ebnf
program     = { function } ;
function    = "fn" ident "(" ")" "->" type "{" expr "}" ;
expr        = primary { "+" primary } ;
primary     = int_lit | bool_lit | call ;
call        = ident "(" ")" ;
type        = "int" | "bool" ;
```

## 4. Type rules

- integer literals have type `int`
- boolean literals have type `bool`
- `a + b` requires `int + int` and returns `int`
- function calls resolve by function name and use declared return type
- function body type must equal declared return type

## 5. Errors

All compiler errors include:
- stable error code (e.g. `E203`)
- message
- byte span

### Error code ranges

- `E001-E099`: lexing
- `E100-E199`: parsing
- `E200-E299`: type checking

## 6. CLI

Required commands:
- `valea check <path>`
- `valea check <path> --json`
- `valea ast <path> --json`
- `valea fmt <path>`

MVP extension:
- `valea emit-c <path>`

## 7. Non-goals (MVP)

- parameters
- local variables
- control flow
- modules/imports
- generics
- macros
- exceptions
- implicit conversions
