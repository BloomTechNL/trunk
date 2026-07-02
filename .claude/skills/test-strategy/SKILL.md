---
description: Testing strategy — hexagonal architecture, screenplay pattern, compile-time DI, and the recipe for adding new dependencies.
---

# Testing Strategy

## Core principle: compile-time dependency injection

Every side effect lives behind a trait. `AppService` is generic over those traits. Production (`main.rs`) passes real implementations; tests (`TestApp`) pass fakes. The command functions receive `&impl Trait` — they never branch on "am I in test mode?" because they don't know. The type system picks the implementation at compile time.

There are zero `#[cfg(test)]` gates in `src/`. There are zero `if testing { fake() } else { real() }` branches. The seam is the generic parameter.

## Adding a new dependency

When new functionality needs a new side effect (e.g. a network call, a clock, a filesystem):

1. **Define the trait** in `src/` with only the methods the command modules need.
   ```rust
   // src/foo.rs
   pub trait Foo {
       fn do_thing(&self, input: &str) -> Result<String>;
   }
   ```

2. **Write the real implementation** in the same module.
   ```rust
   pub struct RealFoo;
   impl Foo for RealFoo { ... }
   ```

3. **Write the fake** in `tests/common/`. It stores state for assertions behind `Cell` or `RefCell` (the trait takes `&self`, so interior mutability is needed).
   ```rust
   // tests/common/fake_foo.rs
   pub struct FakeFoo { pub calls: RefCell<Vec<String>> }
   impl Foo for FakeFoo { ... }
   ```

4. **Thread the generic** through `AppService` and `run_cli`. Add the trait bound, pass it to command functions that need it.
   ```rust
   pub struct AppService<'a, ..., F: Foo> {
       ...
       pub foo: &'a F,
   }
   ```

5. **Add the fake to `TestApp`** as a field, populated in `TestApp::new()`. Expose a convenience method on `TestApp` if the command modules call it directly, or expose it through a screenplay ability.

6. **Wire the real impl** in `main.rs`.

No other files change. The command function signatures gain a `foo: &impl Foo` parameter and delegate to it. That's it.

## BDD and the screenplay pattern

Tests follow BDD: they describe behavior from a **user's perspective**, not the system's internals.

- **Interactions** are things a user does: `Commit`, `Pull`, `WriteFile`. Named as verbs/actions, not as function calls. The test says `bob.attempts_to((Commit { ... },))` — not `app.dispatch_command(Commands::Commit { ... })`.
- **Questions** are things a user observes: `Log`, `Status`. They answer "what would the user see?" — not "what did the system return internally?"

The test reads as a scenario — "Bob writes a file and commits, Alice pulls and sees the commit" — rather than as a sequence of implementation details.

The screenplay pattern is the mechanism that delivers this. Actors with abilities perform interactions and ask questions.

### Building blocks

The `screenplay/` crate provides:

- `Actor` — type-erased map of abilities. Built with `.who_can(...)`.
- `Ability` — marker trait (`trait Ability: 'static {}`). An actor can hold one ability per concrete type.
- `Interaction` — `fn perform_as(&self, actor: &Actor)`. Fetches abilities from the actor and acts.
- `Question<T>` — `fn answered_by(&self, actor: &Actor) -> T`. Fetches abilities and returns a value.
- `Expectation<T>` — `fn test(&self, value: &T) -> bool` + `fn message(&self, value: &T) -> String`.
- `Ensure::that(Q, E)` — wraps a Question + Expectation into an Interaction. Panics on failure.

The framework has zero dependencies and ships no concrete abilities — domain abilities live in `tests/abilities/`.

### Domain abilities (`tests/abilities/`)

| Ability | Purpose |
|---|---|
| `AccessScenarioContext` | Shared `Rc<RefCell<TestContext>>` — the temp dir where `origin.git` lives |
| `UseTrunk` | Owns a `TestApp` — dispatches `g` subcommands |
| `UseGit` | Holds the actor's repo clone path in a `RefCell<PathBuf>` |
| `UseFileSystem` | Marker — signals the actor is allowed to touch the filesystem |

### Domain interactions (`tests/interactions/`)

Each interaction fetches the abilities it needs from the actor and delegates to `TestApp` methods or the git helpers.

```rust
impl Interaction for Commit {
    fn perform_as(&self, actor: &Actor) {
        let trunk = actor.ability::<UseTrunk>().expect("actor needs UseTrunk");
        let git   = actor.ability::<UseGit>().expect("actor needs UseGit");
        trunk.app.commit(&git.repo.borrow(), self.message, self.co_authors.clone())
              .expect("g c should succeed");
    }
}
```

### Domain questions (`tests/questions/`)

Same pattern as interactions, but return a value:

```rust
impl Question<String> for Log {
    fn answered_by(&self, actor: &Actor) -> String {
        let trunk = actor.ability::<UseTrunk>().expect("actor needs UseTrunk");
        let git   = actor.ability::<UseGit>().expect("actor needs UseGit");
        trunk.app.log(&git.repo.borrow())
    }
}
```

### Test structure

```rust
#[test]
fn bob_commits_alice_pulls() {
    let ctx = ScenarioContext::new(TestContext::new());

    let bob = Actor::new()
        .who_can(AccessScenarioContext::new(&ctx))
        .who_can(UseTrunk::new())
        .who_can(UseGit::new())
        .who_can(UseFileSystem);

    let alice = Actor::new()
        .who_can(AccessScenarioContext::new(&ctx))
        .who_can(UseTrunk::new())
        .who_can(UseGit::new())
        .who_can(UseFileSystem);

    // Arrange
    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "bob" },));
    bob.attempts_to((InitialCommit,));
    alice.attempts_to((CloneRepo { name: "alice" },));

    // Act & assert
    bob.attempts_to((
        WriteFile { name: "hello.txt", content: "hello world\n" },
        Commit { message: "add hello.txt", co_authors: vec!["SOLO"] },
        Ensure::that(Log, contains("add hello.txt")),
    ));

    alice.attempts_to((Pull, Ensure::that(Log, contains("add hello.txt"))));
}
```

### Adding a new screenplay interaction or question

1. Add a struct in `tests/interactions/` (or `tests/questions/`).
2. Implement `Interaction` (or `Question<T>`) — fetch abilities from the actor, delegate to `TestApp`.
3. Re-export from the module's `mod.rs`.

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
- `MockFartPlayer` uses `Cell<bool>` for `was_played()`.
- `InMemoryCoAuthorAliases` and `InMemoryTrunkConfig` use `RefCell` for interior mutability.
- All fakes are hand-written. No mocking framework.
- No `Arc`, no `async`. Single-threaded throughout.

## Contract tests for trait implementations

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
| `set_up_basic_repo(base_dir)` | remote + clone + initial commit |

## Existing classic tests

`tests/integration_tests.rs` and `tests/co_author_tests.rs` use the older Arrange-Act-Assert style with direct `TestApp` calls. These remain for coverage but new tests should follow the screenplay pattern above.
