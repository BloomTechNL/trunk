---
description: Enter TDD REFACTOR mode — improve internal structure without changing observable behavior. Use after GREEN.
disable-model-invocation: true
---

# TDD Refactor Mode

You are in the **REFACTOR** phase of test-driven development.

## Your ONLY job

Improve the **internal structure** of the code without changing its observable behavior.

## Allowed
- Rename variables, functions, modules for clarity
- Extract helper functions or methods to eliminate duplication
- Simplify complex expressions
- Reorganize code within a file or across modules
- Add or improve documentation comments
- Remove dead code
- Improve type signatures (e.g., replacing `String` with a more specific type)
- Run `cargo fmt` to auto-format
- Run `cargo test` after **every** change to verify nothing broke
- Run `cargo build` to verify the project builds

## Forbidden
- **Do NOT change behavior** — all existing tests must continue to pass exactly as before
- **Do NOT add new features** — no new functions that aren't extracted from existing code
- **Do NOT write new tests** — not even for coverage gaps you discover (those go in the next RED cycle)
- **Do NOT change test assertions** — tests are the specification; modifying them to "match" your refactor is a bug
- **Do NOT "improve" the test file** — tests are off-limits during refactor (formatting-only changes via `cargo fmt` are OK)

## Success criteria
- `cargo fmt -- --check` passes
- `cargo test` passes with **zero failures**
- Code is measurably better (less duplication, clearer names, simpler structure)

## Refactoring catalog

Safe transformations (cannot change behavior): rename, extract function, inline trivial single-call functions, simplify conditionals, remove dead code, replace magic numbers with named constants.

## When you're done

Stop. You have completed one TDD cycle. The user will start the next cycle with `/red "next_test_name"`.
