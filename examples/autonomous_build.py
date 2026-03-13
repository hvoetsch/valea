#!/usr/bin/env python3
"""
Autonomous App Builder — Valea Live
=====================================
Gives Claude a bold task and a set of real tools (write files, run shell
commands, read files) and watches it autonomously build a complete web
service that has never existed before:

  "Valea Live" — a REST API where you POST business logic written in Valea,
  the compiler validates and executes it, and returns JSON results.

This is the full agent loop:
  design → write → compile → test → fix → iterate → done

Usage:
    cargo build --release
    ANTHROPIC_API_KEY=sk-ant-... .venv/bin/python examples/autonomous_build.py
"""

import anthropic
import json
import os
import subprocess
import textwrap

COMPILER = os.environ.get("VALEA_BIN", "./target/release/valea")
MODEL    = "claude-opus-4-6"
BUILD_DIR = "./build/valea_live"

SYSTEM = textwrap.dedent(f"""
    You are an autonomous software engineer. Your job is to build a complete,
    working application from scratch using the tools available to you.

    You have access to:
    - write_file   : create or overwrite any file
    - read_file    : read any file
    - run_shell    : run any shell command (compiler, pip, python, gcc, curl...)
    - list_files   : list files in a directory

    The Valea compiler is at: {COMPILER}
    Build directory: {BUILD_DIR}

    ## Valea Language (what the compiler accepts)
    - Functions only: fn name() -> type {{ expression }}
    - Types: int (i64), bool
    - Expressions: integer literals, true/false, addition (+), zero-arg calls
    - CLI: `{COMPILER} check file.va --json`  → JSON diagnostics ([] = valid)
    - CLI: `{COMPILER} emit-c file.va`        → C source code

    ## Your mandate
    Build the application completely:
    1. Design the architecture
    2. Write all files
    3. Test everything (actually run it and verify output)
    4. Fix any errors — read them, understand them, fix them
    5. Only declare success when the app actually works end-to-end

    Do not stop at "it should work" — run it and confirm it does.
    Be creative. Build something genuinely useful.
""")

TASK = textwrap.dedent("""
    Build "Valea Live" — a self-contained web service (FastAPI + Python) that
    exposes the Valea compiler as a REST API.

    The service must have these endpoints:

    POST /check
        Body: {{"source": "<valea source code>"}}
        Returns: {{"valid": true/false, "diagnostics": [...]}}

    POST /run
        Body: {{"source": "<valid valea source>", "functions": ["fn1", "fn2"]}}
        Compiles the source, runs each listed function, returns results:
        Returns: {{"results": {{"fn1": 42, "fn2": true}}, "c_source": "..."}}

    GET /
        A minimal HTML page with a textarea to write Valea code and a
        "Run" button that calls /run and shows results. Make it look good.

    GET /health
        Returns: {{"status": "ok", "compiler": "<path>"}}

    Requirements:
    - Everything in ./build/valea_live/
    - One command to start: `python server.py`
    - Runs on port 8765
    - Test it: after starting the server, curl /health and POST /run with a
      real Valea program and verify the results are correct
    - Include a requirements.txt
    - Include a README.md explaining how to run it

    The /run endpoint works by:
    1. Writing the source to a temp .va file
    2. Running `{compiler} check --json` to validate
    3. Running `{compiler} emit-c` to get C code
    4. Appending a C main() that calls each requested function and prints JSON
    5. Compiling with gcc, running, capturing output
    6. Returning the parsed JSON results

    Build it. Test it. Make it work.
""".format(compiler=COMPILER))

# ---------------------------------------------------------------------------
# Tools
# ---------------------------------------------------------------------------

TOOLS = [
    {
        "name": "write_file",
        "description": "Create or overwrite a file with the given content.",
        "input_schema": {
            "type": "object",
            "properties": {
                "path":    {"type": "string", "description": "File path (relative to repo root)"},
                "content": {"type": "string", "description": "File content"},
            },
            "required": ["path", "content"],
        },
    },
    {
        "name": "read_file",
        "description": "Read the content of a file.",
        "input_schema": {
            "type": "object",
            "properties": {
                "path": {"type": "string"},
            },
            "required": ["path"],
        },
    },
    {
        "name": "run_shell",
        "description": (
            "Run a shell command and return stdout + stderr + exit code. "
            "Use this to run the compiler, pip, python, gcc, curl, etc."
        ),
        "input_schema": {
            "type": "object",
            "properties": {
                "command": {"type": "string", "description": "Shell command to run"},
                "timeout": {"type": "integer", "description": "Timeout in seconds (default 30)"},
            },
            "required": ["command"],
        },
    },
    {
        "name": "list_files",
        "description": "List files in a directory.",
        "input_schema": {
            "type": "object",
            "properties": {
                "directory": {"type": "string"},
            },
            "required": ["directory"],
        },
    },
]


def handle_tool(name: str, inputs: dict) -> str:
    if name == "write_file":
        path = inputs["path"]
        os.makedirs(os.path.dirname(path) if os.path.dirname(path) else ".", exist_ok=True)
        with open(path, "w") as f:
            f.write(inputs["content"])
        return f"Written: {path} ({len(inputs['content'])} bytes)"

    elif name == "read_file":
        path = inputs["path"]
        try:
            with open(path) as f:
                content = f.read()
            return content if content else "(empty file)"
        except FileNotFoundError:
            return f"ERROR: file not found: {path}"

    elif name == "run_shell":
        cmd     = inputs["command"]
        timeout = inputs.get("timeout", 30)
        try:
            result = subprocess.run(
                cmd, shell=True, capture_output=True, text=True, timeout=timeout
            )
            output = ""
            if result.stdout.strip():
                output += f"STDOUT:\n{result.stdout}"
            if result.stderr.strip():
                output += f"STDERR:\n{result.stderr}"
            output += f"EXIT CODE: {result.returncode}"
            return output or f"(no output) EXIT CODE: {result.returncode}"
        except subprocess.TimeoutExpired:
            return f"ERROR: command timed out after {timeout}s"
        except Exception as e:
            return f"ERROR: {e}"

    elif name == "list_files":
        directory = inputs["directory"]
        try:
            entries = []
            for root, dirs, files in os.walk(directory):
                dirs[:] = [d for d in dirs if not d.startswith(".") and d != "__pycache__"]
                for file in files:
                    rel = os.path.relpath(os.path.join(root, file), directory)
                    entries.append(rel)
            return "\n".join(sorted(entries)) if entries else "(empty)"
        except FileNotFoundError:
            return f"ERROR: directory not found: {directory}"

    return json.dumps({"error": f"unknown tool: {name}"})


# ---------------------------------------------------------------------------
# Agent loop
# ---------------------------------------------------------------------------

def run() -> None:
    client   = anthropic.Anthropic()
    messages = [{"role": "user", "content": TASK}]

    os.makedirs(BUILD_DIR, exist_ok=True)

    print(f"\n{'═'*66}")
    print("  AUTONOMOUS BUILD — Valea Live")
    print(f"{'═'*66}\n")
    print("Task: build a working FastAPI service that exposes the Valea")
    print(f"compiler as a REST API. Output: {BUILD_DIR}/\n")

    turn = 0
    while True:
        turn += 1
        response = client.messages.create(
            model=MODEL,
            max_tokens=8096,
            thinking={"type": "adaptive"},
            system=SYSTEM,
            tools=TOOLS,
            messages=messages,
        )

        messages.append({"role": "assistant", "content": response.content})

        tool_results = []
        for block in response.content:
            if block.type == "thinking":
                preview = block.thinking[:150].replace("\n", " ")
                print(f"  [thinking] {preview}{'...' if len(block.thinking) > 150 else ''}")
            elif block.type == "text" and block.text.strip():
                print(f"\n{block.text}")
            elif block.type == "tool_use":
                # Summarise the call
                if block.name == "write_file":
                    print(f"\n  ✎  write_file  {block.input['path']}")
                elif block.name == "run_shell":
                    print(f"\n  $  {block.input['command'][:80]}")
                elif block.name == "read_file":
                    print(f"\n  📖  read_file  {block.input['path']}")
                elif block.name == "list_files":
                    print(f"\n  📂  list_files  {block.input['directory']}")

                result = handle_tool(block.name, block.input)

                # Print result (truncated)
                lines = result.splitlines()
                for line in lines[:20]:
                    print(f"     {line}")
                if len(lines) > 20:
                    print(f"     ... ({len(lines)-20} more lines)")

                tool_results.append({
                    "type": "tool_result",
                    "tool_use_id": block.id,
                    "content": result,
                })

        if tool_results:
            messages.append({"role": "user", "content": tool_results})

        if response.stop_reason == "end_turn":
            break

    print(f"\n{'─'*66}")
    print(f"  Build complete — {turn} turns")
    print(f"  Output: {BUILD_DIR}/")
    print(f"  Start:  cd {BUILD_DIR} && pip install -r requirements.txt && python server.py")
    print(f"{'─'*66}\n")


if __name__ == "__main__":
    if not os.path.exists(COMPILER):
        print(f"Compiler not found at '{COMPILER}'.")
        print("Run `cargo build --release` first.")
        raise SystemExit(1)
    run()
