---
description: Testing strategy — hexagonal architecture, screenplay pattern, compile-time DI, and the recipe for adding new dependencies.
---

# Testing Strategy

Every test in this project is one of exactly two kinds:

1. **Screenplay tests** — BDD-style integration tests using `Actor` + `Ability` + `Interaction` + `Question`. These test `g` end-to-end through the `AppService` dispatch path.
2. **Polymorphic adapter tests** — contract tests that run the same suite against both the real and fake implementation of a trait. These test that fakes behave identically to their real counterparts.

## Core principle: compile-time dependency injection

Every side effect lives behind a trait. `AppService` is generic over those traits. Production (`main.rs`) passes real implementations; tests (`TestApp`) pass fakes. The command functions receive `&impl Trait` — they never branch on "am I in test mode?" because they don't know. The type system picks the implementation at compile time.

There are zero `#[cfg(test)]` gates in `src/`. There are zero `if testing { fake() } else { real() }` branches. The seam is the generic parameter.

## Adding a new dependency

When new functionality needs a new side effect (e.g. a network call, a clock, a filesystem):

1. **Define the trait** in `src/` with only the methods the command modules need.
2. **Write the real implementation** in the same module.
3. **Write the fake** in `tests/common/`. It stores state for assertions behind `Cell` or `RefCell` (the trait takes `&self`, so interior mutability is needed).
4. **Thread the generic** through `AppService` and `run_cli`. Add the trait bound, pass it to command functions that need it.
5. **Expose the fake** through either `TestApp` (add a field + method) and/or a screenplay `Ability`. If the new dependency is perceivable by the user (like hearing a fart), create a dedicated `Ability` that wraps shared state from the fake.
6. **Wire the real impl** in `main.rs`.

No other files change. The command function signatures gain a `foo: &impl Foo` parameter and delegate to it. That's it.

## BDD and the screenplay pattern

Tests follow BDD: they describe behavior from a **user's perspective**, not the system's internals.

- **Interactions** are things a user does: `Commit`, `Pull`, `WriteFile`. Named as verbs, not function calls. The test says `bob.attempts_to((Commit { ... },))` — not `app.dispatch_command(Commands::Commit { ... })`.
- **Questions** are things a user observes: `Log`, `Status`, `Diff`. They answer "what would the user see?" — not "what did the system return internally?"

The test reads as a scenario — "Bob writes a file and commits, Kent pulls and sees the commit" — rather than as a sequence of implementation details.

### Screenplay test files

| File | Tests | Focus |
|---|---|---|
| `screenplay_commit_tests.rs` | 2 | Collaboration, commit+delete flow |
| `screenplay_co_author_tests.rs` | 9 | SOLO, aliases, unknown aliases, config-disabled |
| `screenplay_conflict_tests.rs` | 3 | Merge conflict resolve/abort, blocked while conflicted |
| `screenplay_diff_tests.rs` | 3 | Modified file, empty diff, deleted content |
| `screenplay_fart_tests.rs` | 3 | Fart sound, stash-triggered fart, empty stash |
| `screenplay_pull_tests.rs` | 3 | Blocked by unpushed/dirty, clean pull |
| `screenplay_reset_tests.rs` | 1 | Reset clears tracked + untracked |
| `screenplay_revert_tests.rs` | 2 | Revert flow, revert without remote tracking |
| `screenplay_status_tests.rs` | 2 | Untracked files, clean working tree |
| `screenplay_time_travel_tests.rs` | 1 | Time travel blocks writes, `now` restores |

### Building blocks

The `screenplay/` crate provides:

- `Actor` — type-erased map of abilities. Built with `.who_can(...)`.
- `Ability` — marker trait (`trait Ability: 'static {}`). An actor can hold one ability per concrete type.
- `Interaction` — `fn perform_as(&self, actor: &Actor)`. Fetches abilities from the actor and acts.
- `Question<T>` — `fn answered_by(&self, actor: &Actor) -> T`. Fetches abilities and returns a value.
- `Expectation<T>` — `fn test(&self, value: &T) -> bool` + `fn message(&self, value: &T) -> String`. Built-in: `is_true()`, `is_false()`, `equals(T)`, `is_greater_than(T)`, `contains(impl Into<String>)`, `does_not_contain(impl Into<String>)`.
- `Ensure::that(Q, E)` — wraps a Question + Expectation into an Interaction. Panics on failure.
- `Outcome` — `Success` | `Failure(String)`. Produced by `doing()`.
- `doing(interaction)` — wraps an `Interaction` in a `Question<Outcome>`. Uses `catch_unwind` to convert panics into `Outcome::Failure`.
- `fails()` — `Expectation<Outcome>`. Matches any failure. `fails().with_error("...")` matches a failure containing a substring.
- `equals(String::new())` — `Expectation<String>`. Matches an exact empty string (used for "diff is empty" assertions).

The framework has zero dependencies and ships no concrete abilities — domain abilities live in `tests/abilities/`.

### Domain abilities (`tests/abilities/`)

| Ability | Purpose |
|---|---|
| `AccessScenarioContext` | Shared `Rc<RefCell<TestContext>>` — the temp dir where `origin.git` lives |
| `UseTrunk` | Owns a `TestApp` — dispatches `g` subcommands |
| `UseGit` | Holds the actor's repo clone path in a `RefCell<PathBuf>` |
| `UseFileSystem` | Marker — signals the actor is allowed to touch the filesystem |
| `Hear` | Holds `Rc<Cell<bool>>` shared with `MockFartPlayer` — allows asserting on fart sounds without depending on `UseTrunk` |

### Domain interactions (`tests/interactions/`)

Each interaction fetches the abilities it needs from the actor and delegates to `TestApp` methods or the git helpers.

| Interaction | Abilities needed | Wraps |
|---|---|---|
| `AbortCommit` | `UseTrunk`, `UseGit` | `TestApp::commit_abort()` |
| `AddAlias` | `UseTrunk` | `TestApp::add_alias()` |
| `CloneRepo { name }` | `AccessScenarioContext`, `UseFileSystem`, `UseGit` | `use_git::clone_repo()` |
| `Commit { message, co_authors }` | `UseTrunk`, `UseGit` | `TestApp::commit()` — panics on failure |
| `CommitUnpushed` | `UseFileSystem`, `UseGit` | `use_git::commit_file()` |
| `Config { co_authors_required }` | `UseTrunk`, `UseGit` | `TestApp::config()` |
| `CreateDir { name }` | `UseFileSystem`, `UseGit` | `fs::create_dir()` |
| `DeleteFile { name }` | `UseFileSystem`, `UseGit` | `fs::remove_file()` |
| `Fart` | `UseTrunk`, `UseGit` | `TestApp::fart()` |
| `InitialCommit` | `UseFileSystem`, `UseGit` | `use_git::initial_commit()` |
| `Pull` | `UseTrunk`, `UseGit` | `TestApp::pull()` |
| `PutInStash` | `UseFileSystem`, `UseGit` | `use_git::put_something_in_stash()` |
| `Reset` | `UseTrunk`, `UseGit` | `TestApp::reset()` |
| `ResolveCommit` | `UseTrunk`, `UseGit` | `TestApp::commit_resolve()` |
| `RevertHead` | `UseTrunk`, `UseGit` | `TestApp::revert()` — reverts newest commit |
| `SetUpRemote` | `AccessScenarioContext`, `UseFileSystem` | `use_git::set_up_remote()` |
| `TimeTravel { target }` | `UseTrunk`, `UseGit` | `TestApp::time_travel()` |
| `WriteFile { name, content }` | `UseFileSystem`, `UseGit` | `common::write_file::write_file()` |

### Domain questions (`tests/questions/`)

Same pattern as interactions, but return a value:

| Question | Returns | Abilities needed |
|---|---|---|
| `CommitHashes` | `Vec<String>` | `UseTrunk`, `UseGit` |
| `Diff` | `String` | `UseTrunk`, `UseGit` |
| `FileContent { name }` | `String` | `UseGit` |
| `FileExists { name }` | `bool` | `UseGit` |
| `HeardFart` | `bool` | `Hear` |
| `Log` | `String` | `UseTrunk`, `UseGit` |
| `Status` | `String` | `UseTrunk`, `UseGit` |

### Cast (`tests/cast/`)

Factory functions that create pre-equipped actors. Every test uses these — never `Actor::new().who_can(...)` directly.

```rust
pub fn developer_bob(ctx: &ScenarioContext) -> Actor { ... }
pub fn developer_kent(ctx: &ScenarioContext) -> Actor { ... }
```

Both equip the actor with: `AccessScenarioContext`, `UseTrunk`, `UseGit`, `UseFileSystem`, `Hear`.

The variable name matches the actor name: `let bob = developer_bob(&ctx)`.

### Happy path vs. error path

For tests that expect an interaction to succeed, use the interaction directly:
```rust
bob.attempts_to((Commit { message: "...", co_authors: vec!["SOLO"] },));
```

For tests that expect an interaction to fail, wrap it in `doing()` and assert with `fails()`:
```rust
bob.attempts_to((
    Ensure::that(
        doing(Commit { message: "...", co_authors: vec![] }),
        fails().with_error("You must either specify co-authors"),
    ),
));
```

This reuses the same `Commit` interaction for both cases — `doing()` catches the panic from `.expect()` via `catch_unwind`.

### Test structure

```rust
#[test]
fn bob_commits_kent_pulls() {
    let ctx = ScenarioContext::new(TestContext::new());

    let bob = developer_bob(&ctx);
    let kent = developer_kent(&ctx);

    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "bob" },));
    bob.attempts_to((InitialCommit,));

    kent.attempts_to((CloneRepo { name: "kent" },));

    bob.attempts_to((
        WriteFile { name: "hello.txt", content: "hello world\n" },
        Commit { message: "add hello.txt", co_authors: vec!["SOLO"] },
        Ensure::that(Log, contains("add hello.txt")),
    ));

    kent.attempts_to((Pull, Ensure::that(Log, contains("add hello.txt"))));

    bob.attempts_to((Ensure::that(Status, contains("nothing to commit")),));
}
```

### Adding a new screenplay interaction or question

1. Add a struct in `tests/interactions/` (or `tests/questions/`).
2. Implement `Interaction` (or `Question<T>`) — fetch abilities from the actor, delegate to `TestApp` or git helpers.
3. Re-export from the module's `mod.rs`.

Name interactions so they read naturally in `attempts_to(...)`: `AbortCommit` not `CommitAbort`, `ResolveCommit` not `CommitResolve`.

## TestApp — composition root for fakes

`tests/common/test_app.rs` owns all fakes and a `TempDir`. It exposes convenience methods that construct `Cli` values and call `AppService::dispatch_command`.

```rust
pub struct TestApp {
    pub base_dir: TempDir,
    fart_player: MockFartPlayer,
    co_author_aliases: InMemoryCoAuthorAliases,
    trunk_config: InMemoryTrunkConfig,
    output: CapturingSink,
}
```

Key points:
- `CapturingSink` stores output in a `Mutex<String>`. Its `take()` method drains the buffer — call once per assertion to isolate output.
- `MockFartPlayer` uses `Rc<Cell<bool>>` internally. `inner()` returns a clone of the `Rc` so that `Hear` can share the same flag.
- `InMemoryCoAuthorAliases` and `InMemoryTrunkConfig` use `RefCell` for interior mutability.
- All fakes are hand-written. No mocking framework.
- No `Arc`, no `async`. Single-threaded throughout.

## Polymorphic adapter tests (contract tests)

`tests/test_co_author_aliases.rs` and `tests/trunk_config_tests.rs` use a macro to run the same tests against both the real and fake implementation of a trait:

```rust
macro_rules! aliases_test_suite {
    ($init_macro:ident) => {
        mod $init_macro {
            // tests use $init_macro!(aliases) to create the impl under test
        }
    };
}

aliases_test_suite!(create_real_co_author_aliases);
aliases_test_suite!(create_in_memory_co_author_aliases);
```

Every new trait should get a contract test suite like this. Additional tests specific to the real implementation (file I/O, error handling) go in a separate module.

## Git fixture helpers (`tests/common/use_git.rs`)

These spawn real `git` subprocesses — they are not behind a trait. They set up repos on disk with `GIT_EDITOR=true` and `GIT_TERMINAL_PROMPT=0`.

| Function | Does |
|---|---|
| `set_up_remote(base_dir)` | `git init --bare origin.git` |
| `clone_repo(base_dir, name, from)` | Clone, configure identity, return `PathBuf` |
| `initial_commit(repo_dir)` | README + add + commit + push |
| `put_something_in_stash(repo_dir)` | Write file, add, stash |
| `commit_file(repo_dir)` | Write file, add, commit — no push |
| `set_up_basic_repo(base_dir)` | remote + clone + initial commit |
