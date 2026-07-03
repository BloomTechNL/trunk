---
description: Testing strategy — hexagonal architecture, screenplay pattern, compile-time DI, and the recipe for adding new dependencies.
---

# Testing Strategy

Every test in this project is one of exactly two kinds:

1. **Screenplay tests** — BDD-style integration tests using `Actor` + `Ability` + `Interaction` + `Question`. These test `g` end-to-end through the `AppService` dispatch path.
2. **Polymorphic adapter tests** — contract tests that run the same suite against both the real and fake implementation of a trait.

## Core principle: compile-time dependency injection

Every side effect lives behind a trait. `AppService` is generic over those traits. Production passes real implementations; tests pass fakes. Command functions receive `&impl Trait` — they never branch on test mode. Zero `#[cfg(test)]` gates in `src/`. The type system picks the implementation at compile time.

## Recipes

### Adding a new dependency

When new functionality needs a new side effect (e.g., a network call, a clock):

1. **Define the trait** in `src/` with only the methods command modules need.
2. **Write the real implementation** in the same module.
3. **Write the fake** in `tests/common/`. Store state for assertions behind `Cell` or `RefCell` (the trait takes `&self`, so interior mutability is needed).
4. **Thread the generic** through `AppService` and `run_cli` — pattern in `src/cli.rs`. Add the trait bound, pass it to command functions.
5. **Expose the fake** through `TestApp` (add a field + method) and/or a screenplay `Ability`. If the dependency is perceivable by the user, create a dedicated `Ability` wrapping shared state from the fake — see `Hear` in `tests/abilities/hear.rs`.
6. **Wire the real impl** in `main.rs`.

### Adding a screenplay interaction

1. Add a struct in `tests/interactions/`.
2. Implement `Interaction` — fetch abilities from the actor, delegate to `TestApp` or git helpers. Pattern: see `tests/interactions/commit.rs`.
3. Re-export from the module's `mod.rs`.

Name interactions so they read naturally: `AbortCommit` not `CommitAbort`.

### Adding a screenplay question

1. Add a struct in `tests/questions/`.
2. Implement `Question<T>` — fetch abilities, return a value. Pattern: see `tests/questions/log.rs`.
3. Re-export from the module's `mod.rs`.

### Setting up contract tests

1. Write a macro that takes an init macro as argument — pattern: `tests/test_co_author_aliases.rs`.
2. Inside the macro, write the full trait contract as tests. Every behavior the app depends on must be covered.
3. Invoke the macro once for each implementation (real + fake).
4. No separate test modules per implementation — the shared suite is the single source of truth.

### Adding a new acceptance test

1. Create a file under `tests/acceptance/`.
2. Add `mod new_test_file;` to `tests/acceptance_tests.rs`.
3. Use `crate::` prefix for imports (`use crate::abilities::...`).
4. Follow the test structure skeleton below.

## Key patterns

### Happy path vs. error path

For happy-path tests, use the interaction directly:

```rust
bob.attempts_to((Commit { message: "...", co_authors: vec!["SOLO"] },));
```

For error-path tests, wrap in `doing()` and assert with `fails()`:

```rust
bob.attempts_to((
    Ensure::that(
        doing(Commit { message: "...", co_authors: vec![] }),
        fails().with_error("You must either specify co-authors"),
    ),
));
```

This reuses the same `Commit` interaction for both cases — `doing()` catches the panic from `.expect()` via `catch_unwind`.

### Test structure skeleton

```rust
#[test]
fn scenario_name() {
    let ctx = ScenarioContext::new(TestContext::new());
    let bob = developer_bob(&ctx);
    let kent = developer_kent(&ctx);

    // setup
    // exercise + verify
}
```

Pattern: create actors via cast factories (never `Actor::new()` directly), set up state, exercise the system, verify with `Ensure::that(Question, Expectation)`. For a full example, see `tests/acceptance/commit_tests.rs`.

### TestApp — composition root

`tests/common/test_app.rs` owns all fakes and a `TempDir`. It exposes convenience methods that construct `Cli` values and call `AppService::dispatch_command`.

Key points:
- `CapturingSink::take()` drains the output buffer — call once per assertion.
- `MockFartPlayer::inner()` returns `Rc<Cell<bool>>` — share with `Hear` ability for fart assertions.
- All fakes are hand-written (no mocking framework). Single-threaded, no `Arc`.

### Cast factories

Always use `developer_bob(&ctx)` / `developer_kent(&ctx)` from `tests/cast/`. Never call `Actor::new().who_can(...)` directly.

## Screenplay framework

The `screenplay/` crate (zero dependencies) provides: `Actor`, `Ability`, `Interaction`, `Question<T>`, `Expectation<T>`, `Ensure::that(Q, E)`, `Outcome` (`Success` | `Failure(String)`), `doing(interaction)`, `fails()`, `contains()`, `does_not_contain()`, `equals()`.

Domain abilities, interactions, and questions live in `tests/` — the framework ships no concrete implementations.
