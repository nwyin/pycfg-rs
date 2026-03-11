# Static Analysis for Agents

## Purpose

Use static analysis when the task is structural rather than behavioral.

Static analysis is good at answering:

- what code exists here?
- what functions, classes, or symbols are relevant?
- how does control flow through this function?
- what code is likely affected by this edit?
- what should be inspected before making a change?

Static analysis is not a substitute for runtime validation. It narrows the
search space and improves editing decisions; tests and execution still decide
whether behavior is correct.

## When to Use It

Use static analysis early for:

- unfamiliar codebases
- medium or large repositories
- refactors
- risky feature work in existing code
- review and impact analysis
- migration and cleanup work
- debugging where the first problem is "where should I look?"

It is especially useful before editing:

- branch-heavy functions
- loop-heavy code
- `try` / `except` / `finally` logic
- code with multiple returns or exits
- code that spans multiple modules or ownership boundaries

## When It Helps Less

Static analysis is weaker for:

- runtime configuration issues
- data-dependent bugs
- timing and concurrency bugs
- reflection, monkeypatching, or heavy dynamic behavior
- exact runtime values and environment-driven behavior

In those cases, use static analysis only for orientation, then move quickly to
tests, logs, tracing, or direct execution.

## Recommended Tool Order

Choose the narrowest tool that answers the question.

### 1. `rg`

Use for:

- literal search
- symbol inventory
- pattern search
- quick file discovery

This is the fastest first pass when the question is textual.

### 2. Call-graph or symbol tools

Use for:

- callers and callees
- dependency mapping
- impact analysis across functions or modules
- finding the edit surface for a refactor

These answer cross-function questions.

### 3. `pycfg-rs`

Use for:

- branches inside one function
- loop exits and back-edges
- return paths
- exception routing
- structural complexity

This answers function-level control-flow questions.

### 4. Tests and runtime commands

Use for:

- behavioral validation
- bug confirmation
- regression checking
- runtime truth

This is the final authority on actual behavior.

## `pycfg-rs` Guidance

Use `pycfg-rs` when the question is about intra-procedural structure in Python.

Good fits:

- "How many exits does this function have?"
- "Does this loop always reach the same merge point?"
- "What happens on `break`, `continue`, or `return`?"
- "Does `finally` run on every abrupt path?"
- "What exact function should I inspect before editing this branchy logic?"

High-value commands:

```bash
# Discover exact qualified names
pycfg --list-functions path/to/file.py

# Quick sizing without dumping full CFG blocks
pycfg --summary path/to/file.py
pycfg --summary --format json path/to/file.py::Qualified.name

# Full CFG for machine reasoning
pycfg --format json path/to/file.py::Qualified.name

# Human-readable CFG
pycfg path/to/file.py::Qualified.name

# Parse health
pycfg --diagnostics --format json path/to/file.py

# More detailed exception routing
pycfg --explicit-exceptions path/to/file.py::Qualified.name
```

Important facts:

- Function targets are exact qualified names.
- Directory inputs recurse over `.py` files.
- Non-`.py` files are ignored.
- Parse-broken files are skipped during CFG generation.
- `--diagnostics` reports parse issues directly.
- Async constructs are flattened to synchronous structure.
- `yield` and `yield from` are preserved as statements, not modeled as
  suspension points.

## Agent Workflow

For code editing agents, a good default sequence is:

1. Use `rg` to find likely files and symbols.
2. Use a call-graph or symbol tool if the question crosses function boundaries.
3. Use `pycfg-rs` if the risky part is inside a Python function.
4. Make the code change.
5. Re-run static analysis if the change is structurally sensitive.
6. Run tests or the program itself to validate behavior.

## Policy

Good default policy:

- require static analysis before editing unfamiliar code
- require static analysis before large refactors
- require CFG inspection before editing complex Python control flow
- do not rely on static analysis alone to close a bug
- always follow with tests or runtime validation when behavior matters

## Prompt Block

```text
Use static analysis first when the task is structural: discovery, scoping,
dependency mapping, control-flow understanding, or refactoring risk.

Choose the narrowest tool that answers the question:
- use `rg` for literal search and file discovery
- use call-graph or symbol tools for cross-function relationships
- use `pycfg-rs` for Python function-level control flow
- use tests and runtime commands for behavioral validation

Do not treat static analysis as proof of runtime behavior.
Use it to narrow the search space, identify likely impact, and choose what to
inspect or test next.

For Python control-flow questions:
- use `pycfg --list-functions` to discover exact qualified names
- use `pycfg --summary` for quick sizing
- use `pycfg --format json file.py::Qualified.name` for exact CFG reasoning
- use `pycfg --diagnostics` if parsing may be failing
- use `pycfg --explicit-exceptions` only when exception routing detail matters
```
