---
description: Enter TDD GREEN mode — write minimal implementation to make the failing test pass. Use after RED.
disable-model-invocation: true
---

# TDD Green Mode

You are in the **GREEN** phase of test-driven development.

## Your ONLY job

Write the **minimal implementation** to make the failing test pass.

## Allowed
- Read the failing test to understand what behavior is expected
- Read existing code to understand where the implementation should go
- Write the smallest amount of implementation code that makes the test pass
- Run `cargo test` to verify all tests pass
- Run `cargo build` to verify the project builds

## Forbidden
- **Do NOT refactor anything** — no renaming, extracting helpers, cleaning up, or "improving" existing code. That's for REFACTOR mode.
- **Do NOT add extra features** — implement ONLY what the test requires. No "while I'm here" additions.
- **Do NOT write new tests** — not even for edge cases you notice. Those go in the next RED cycle.
- **Do NOT change the test** — the test is the specification. If the test is wrong, stop and tell the user.
- **Do NOT touch code unrelated to the failing test** — even if you see problems nearby

## Success criteria
- `cargo test` passes with **zero failures**
- Only the minimum code was written to achieve that

## Anti-pattern: "test-first temptation"
When a test is well-written, the implementation is often trivial — sometimes a single line. That is **correct**. You are not being asked to "do more" or "be thorough." You are being asked to make the test pass with the least code possible.

## When you're done

Stop. Do not proceed to REFACTOR mode. The user will invoke `/refactor` when they're ready.
