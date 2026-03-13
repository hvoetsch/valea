#!/usr/bin/env python3
"""
Valea Agent Loop Demo
=====================
Demonstrates the AI-agent feedback loop that Valea is designed for.

An agent generates a program, the compiler validates it and returns structured
JSON diagnostics, the agent reads the error codes, fixes the source, and
iterates until the program is correct — then emits C.

This is the core value proposition of an AI-native language: deterministic
error codes + machine-readable output = autonomous repair without human help.

Usage (after `cargo install --path .` in the repo root):
    python3 examples/agent_loop.py
"""

import json
import os
import subprocess
import tempfile

COMPILER = os.environ.get("VALEA_BIN", "valea")

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

def check(source: str) -> tuple[bool, list[dict]]:
    """Compile-check a source string. Returns (ok, diagnostics)."""
    path = _write_temp(source)
    try:
        r = subprocess.run([COMPILER, "check", path, "--json"],
                           capture_output=True, text=True)
        return r.returncode == 0, json.loads(r.stdout)
    finally:
        os.unlink(path)


def get_ast(source: str) -> dict:
    """Return the AST as a Python dict."""
    path = _write_temp(source)
    try:
        r = subprocess.run([COMPILER, "ast", path, "--json"],
                           capture_output=True, text=True)
        return json.loads(r.stdout)
    finally:
        os.unlink(path)


def emit_c(source: str) -> str:
    """Return the emitted C code for a valid program."""
    path = _write_temp(source)
    try:
        r = subprocess.run([COMPILER, "emit-c", path],
                           capture_output=True, text=True)
        return r.stdout
    finally:
        os.unlink(path)


def fmt(source: str) -> str:
    """Return the canonically formatted version of the source."""
    path = _write_temp(source)
    try:
        subprocess.run([COMPILER, "fmt", path], check=True)
        with open(path) as f:
            return f.read()
    finally:
        os.unlink(path)


def _write_temp(source: str) -> str:
    fd, path = tempfile.mkstemp(suffix=".va")
    with os.fdopen(fd, "w") as f:
        f.write(source)
    return path


# ---------------------------------------------------------------------------
# Agent knowledge base: how to fix each error code
# ---------------------------------------------------------------------------

FIXES = {
    "E200": "Remove duplicate function definition",
    "E201": "Return type mismatch — fix the body expression to match the declared type",
    "E202": "Call to unknown function — define the missing function first",
    "E203": "'+' requires int operands — replace bool operand with an int expression",
}

# ---------------------------------------------------------------------------
# Demo
# ---------------------------------------------------------------------------

def section(title: str):
    print(f"\n{'=' * 60}")
    print(f"  {title}")
    print('=' * 60)


def show_diags(diags: list[dict]):
    for d in diags:
        hint = FIXES.get(d["code"], "unknown error")
        print(f"  [{d['code']}] {d['message']}")
        print(f"         → agent hint: {hint}")
        print(f"         → span: {d['span']['start']}..{d['span']['end']}")


# ── Round 1: type error (E201) ──────────────────────────────────────────────
section("Round 1 — Agent generates a program with a type error")

attempt = """\
fn is_positive() -> bool {
    42
}
"""
print(f"\nSource:\n{attempt}")
ok, diags = check(attempt)
print(f"Result: {'✓ ok' if ok else f'✗ {len(diags)} error(s)'}")
show_diags(diags)

print("\nAgent reads E201: body returns int but function declares bool.")
print("Fix: change return type to int  (agent chooses minimal change).")

attempt = """\
fn is_positive() -> int {
    42
}
"""
ok, diags = check(attempt)
print(f"\nAfter fix: {'✓ ok' if ok else '✗ still failing'}")

if ok:
    print("\nAST (machine-readable, agent can introspect the program structure):")
    print(json.dumps(get_ast(attempt), indent=2))

# ── Round 2: unknown function call (E202) ───────────────────────────────────
section("Round 2 — Agent generates a call to an undefined function")

attempt = """\
fn total() -> int {
    base() + bonus()
}
"""
print(f"\nSource:\n{attempt}")
ok, diags = check(attempt)
print(f"Result: {'✓ ok' if ok else f'✗ {len(diags)} error(s)'}")
show_diags(diags)

print("\nAgent reads E202 ×2: base() and bonus() are undefined.")
print("Fix: add stub definitions for both.")

attempt = """\
fn base() -> int {
    100
}

fn bonus() -> int {
    25
}

fn total() -> int {
    base() + bonus()
}
"""
ok, diags = check(attempt)
print(f"\nAfter fix: {'✓ ok' if ok else '✗ still failing'}")
if ok:
    print("\nEmitting C (forward declarations ensure any call order compiles):")
    print(emit_c(attempt))

# ── Round 3: Fibonacci via function chaining ────────────────────────────────
section("Round 3 — Agent builds Fibonacci via pure function composition")

fib = """\
fn fib1() -> int { 1 }
fn fib2() -> int { 1 }
fn fib3() -> int { fib1() + fib2() }
fn fib4() -> int { fib2() + fib3() }
fn fib5() -> int { fib3() + fib4() }
fn fib6() -> int { fib4() + fib5() }
fn fib7() -> int { fib5() + fib6() }
fn fib8() -> int { fib6() + fib7() }
fn fib9() -> int { fib7() + fib8() }
fn fib10() -> int { fib8() + fib9() }
"""
print(f"\nSource:\n{fib}")
ok, diags = check(fib)
print(f"Check: {'✓ ok' if ok else f'✗ {len(diags)} error(s)'}")

print("\nCanonical format (agent always formats before committing):")
print(fmt(fib))

print("\nEmitted C:")
print(emit_c(fib))

# ── Summary ──────────────────────────────────────────────────────────────────
section("Summary — Why this matters for AI-native development")
print("""
  1. Stable error codes (E201, E202 …) let agents branch on error type,
     not fragile regex over human-readable messages.

  2. JSON diagnostics with source spans enable automated repair loops
     without screen-scraping compiler output.

  3. Canonical formatter + deterministic output means two agents generating
     the same program always produce identical text — no diff noise.

  4. Forward declarations in emitted C mean agent-generated call order
     never causes downstream C compilation failures.

  Everything is designed so an agent can: generate → validate → repair → emit
  without ever needing a human in the loop.
""")
