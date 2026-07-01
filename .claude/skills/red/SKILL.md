---
description: Enter TDD RED mode — write a failing test, nothing more. Use when starting a new TDD cycle.
disable-model-invocation: true
argument-hint: "[test_name]"
---

# TDD Red Mode

You are in the **RED** phase of test-driven development.

Target test: **$ARGUMENTS**

## Your ONLY job

Write a **failing test** — nothing more.

## Allowed
- Read existing code to understand the interface you're testing
- Write a new test (or add a test case to an existing test) that exercises behavior that does **not exist yet**
- Run `cargo check` to verify the test compiles
- Run `cargo build` to verify the project builds with the new test

## Forbidden
- **Do NOT write any implementation code** — no `impl` blocks, no new functions in `src/`, no method bodies beyond `todo!()` or `unimplemented!()`
- **Do NOT fix existing tests** — if other tests break because of your changes, stop and reconsider
- **Do NOT make the test pass** — the test must FAIL. That is the point of RED mode.
- **Do NOT refactor anything** — even if you see ugly code
- **Do NOT run `cargo test`** (except to verify your test name is correct and it fails for the expected reason)

## Success criteria
- `cargo build` succeeds
- The targeted test **fails** — ideally for the expected reason (the feature isn't implemented yet), not due to a compilation error or wrong test name

## Failure modes to avoid
- Writing a test that passes immediately → you wrote a test for existing behavior, not new behavior
- Writing implementation code "just so the test compiles" → that's GREEN mode work
- Fixing unrelated compiler errors in `src/` → those should be handled in a different mode

## When you're done

Stop. Do not proceed to GREEN mode. The user will invoke `/green` when they're ready.
