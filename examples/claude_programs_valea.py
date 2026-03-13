#!/usr/bin/env python3
"""
Claude Programs in Valea — Autonomous Agent Demo
=================================================
Uses the Claude API with tool use so Claude autonomously:
  1. Writes a Valea program to solve a given task
  2. Checks it using the compiler (JSON diagnostics)
  3. Reads the error codes and fixes the program
  4. Repeats until it passes, then emits C

This is the full AI-native language vision: structured compiler feedback
drives an autonomous repair loop without any human intervention.

Usage:
    pip install anthropic
    cargo build --release          # build the compiler first
    ANTHROPIC_API_KEY=sk-... python examples/claude_programs_valea.py
"""

import anthropic
import json
import os
import subprocess
import tempfile

# ---------------------------------------------------------------------------
# Configuration
# ---------------------------------------------------------------------------

COMPILER = os.environ.get("VALEA_BIN", "./target/release/valea")
MODEL = "claude-opus-4-6"

SYSTEM_PROMPT = """\
You are an expert programmer working in the Valea language.

## Valea Language Rules (MVP — these are ALL the rules, nothing else exists)

Functions:
    fn name() -> type {
        expression
    }

Types: int (i64), bool
Expressions: integer literals, true/false, addition (+), zero-arg function calls
NO parameters, NO variables, NO control flow, NO loops, NO imports.

## Error Code Reference
E201 — return type mismatch (body type ≠ declared type)
E202 — call to unknown function (define it first)
E200 — duplicate function name
E203 — '+' used with non-int operands

## Canonical Format
- 4-space indent in body
- Blank line between functions
- fn name() -> type {\\n    expr\\n}

## Workflow
1. Use check_program to validate your code (always returns JSON).
2. If errors exist, read the error codes, fix the source, and check again.
3. Once check_program returns [], use emit_c to get the C output.
4. Summarise what you built and show the final C.
"""

# ---------------------------------------------------------------------------
# Compiler tools
# ---------------------------------------------------------------------------

TOOLS = [
    {
        "name": "check_program",
        "description": (
            "Validate a Valea source program. "
            "Returns a JSON array of diagnostics. "
            "An empty array [] means the program is valid. "
            "Each diagnostic has: code (E###), message, span.start, span.end."
        ),
        "input_schema": {
            "type": "object",
            "properties": {
                "source": {
                    "type": "string",
                    "description": "The complete Valea source code to validate.",
                }
            },
            "required": ["source"],
        },
    },
    {
        "name": "emit_c",
        "description": (
            "Emit C code for a valid Valea program. "
            "The program must already pass check_program (empty diagnostics). "
            "Returns the generated C source as a string."
        ),
        "input_schema": {
            "type": "object",
            "properties": {
                "source": {
                    "type": "string",
                    "description": "The validated Valea source code.",
                }
            },
            "required": ["source"],
        },
    },
    {
        "name": "get_ast",
        "description": (
            "Return the AST of a Valea program as JSON. "
            "Useful for introspecting program structure."
        ),
        "input_schema": {
            "type": "object",
            "properties": {
                "source": {"type": "string"}
            },
            "required": ["source"],
        },
    },
]


def _run(args: list[str], source: str) -> tuple[str, int]:
    """Write source to a temp file, run the compiler, return (stdout, returncode)."""
    fd, path = tempfile.mkstemp(suffix=".va")
    try:
        with os.fdopen(fd, "w") as f:
            f.write(source)
        result = subprocess.run(args + [path], capture_output=True, text=True)
        return result.stdout.strip(), result.returncode
    finally:
        os.unlink(path)


def handle_tool(name: str, inputs: dict) -> str:
    source = inputs["source"]
    if name == "check_program":
        out, _ = _run([COMPILER, "check", "--json"], source)
        return out or "[]"
    elif name == "emit_c":
        out, rc = _run([COMPILER, "emit-c"], source)
        if rc != 0:
            # Compiler returned errors — pass them back as a hint
            return json.dumps({"error": "program has errors", "details": out})
        return out
    elif name == "get_ast":
        out, _ = _run([COMPILER, "ast", "--json"], source)
        return out or "{}"
    return json.dumps({"error": f"unknown tool: {name}"})


# ---------------------------------------------------------------------------
# Agentic loop
# ---------------------------------------------------------------------------

def run_task(task: str) -> None:
    client = anthropic.Anthropic()
    messages = [{"role": "user", "content": task}]

    print(f"\n{'═' * 64}")
    print(f"  TASK: {task}")
    print(f"{'═' * 64}\n")

    turn = 0
    while True:
        turn += 1
        response = client.messages.create(
            model=MODEL,
            max_tokens=4096,
            thinking={"type": "adaptive"},
            system=SYSTEM_PROMPT,
            tools=TOOLS,
            messages=messages,
        )

        # Collect assistant message for history
        messages.append({"role": "assistant", "content": response.content})

        tool_results = []
        for block in response.content:
            if block.type == "thinking":
                print(f"[thinking] {block.thinking[:200]}{'...' if len(block.thinking) > 200 else ''}\n")
            elif block.type == "text" and block.text.strip():
                print(block.text)
            elif block.type == "tool_use":
                print(f"\n── Tool call: {block.name} (turn {turn})")
                result = handle_tool(block.name, block.input)

                # Pretty-print short results, truncate long ones
                try:
                    parsed = json.loads(result)
                    pretty = json.dumps(parsed, indent=2)
                    display = pretty[:600] + "\n  ..." if len(pretty) > 600 else pretty
                except (json.JSONDecodeError, TypeError):
                    display = result[:600] + "\n  ..." if len(result) > 600 else result
                print(f"   Result: {display}\n")

                tool_results.append({
                    "type": "tool_result",
                    "tool_use_id": block.id,
                    "content": result,
                })

        if tool_results:
            messages.append({"role": "user", "content": tool_results})

        if response.stop_reason == "end_turn":
            break

    print(f"\n{'─' * 64}")
    print(f"  Done in {turn} turn(s).")
    print(f"{'─' * 64}\n")


# ---------------------------------------------------------------------------
# Tasks
# ---------------------------------------------------------------------------

TASKS = [
    # Task 1: Intentionally introduces a type error — agent must fix it
    (
        "Write a Valea program with these functions:\n"
        "  - one() returns the integer 1\n"
        "  - two() returns 1 + 1\n"
        "  - is_two_even() returns bool true\n"
        "Make sure all return types are correct. "
        "Check the program, fix any errors, then emit C."
    ),

    # Task 2: Fibonacci chain — agent figures out the composition pattern
    (
        "Build a Valea program that computes the 8th Fibonacci number.\n"
        "The language has no loops or recursion, so use function chaining:\n"
        "fib1=1, fib2=1, fib3=fib1+fib2, fib4=fib2+fib3, etc.\n"
        "Check, fix if needed, then emit C."
    ),

    # Task 3: Agent discovers it needs to define dependencies first
    (
        "Write a Valea program where:\n"
        "  - salary()    returns 5000\n"
        "  - bonus()     returns 1000\n"
        "  - tax()       returns 800\n"
        "  - net_pay()   returns salary + bonus - tax\n"
        "Note: '-' is not supported in Valea — find a way to express this "
        "within the language constraints. Check the result."
    ),
]


if __name__ == "__main__":
    # Check the compiler is available
    if not os.path.exists(COMPILER):
        print(f"Compiler not found at '{COMPILER}'.")
        print("Run `cargo build --release` in the repo root first.")
        raise SystemExit(1)

    for task in TASKS:
        run_task(task)
