# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build, Test, Lint

```bash
cargo build                    # debug build
cargo build --release          # release build (binary at target/release/g)
cargo test -q                    # run all tests
cargo test -q --test integration_tests -- test_clean_commit_flow  # single test
cargo check                    # fast compile-check (no codegen)
cargo fmt -- --check           # verify formatting
cargo fmt                      # auto-format
```

The pre-commit hook (installed via `scripts/set_up_dev.sh`, which runs `git config core.hooksPath .git_hooks`) does `cargo check && cargo test && cargo fmt -- --check`.

## Architecture

`g` is a deliberately narrow git wrapper for trunk-based development. `src/git.rs` provides the passthrough/capture helpers (`git_passthrough`, `git_capture`, `git_capture_silent`).

### When to use `git2` vs shelling out to `git`

**Use `git2` for all local repository operations** — repo introspection, index inspection, diffing, status checks, conflict detection, etc. Examples: `time_travel.rs` revwalk, `reset.rs` status-based clean-up, `has_stash.rs` stash detection, `has_conflict_markers`.

**Shell out to `git` only in two cases:**
1. **Network I/O** — push, pull, fetch, clone (libgit2 handles these poorly).
2. **Passing through output to the user** — when the user needs to see raw git output (log, status, diff display).

This means `git_passthrough` / `git_capture` should be reserved for those two cases; new local operations should prefer `git2`.

### Trait-based dependency injection

Three traits decouple the CLI from side effects, making the entire app testable without real git repos or audio:

- **`OutputSink`** (`src/output.rs`) — abstracts stdout. `StdoutSink` in production; `CapturingSink` in tests accumulates output into a `Mutex<String>` for assertions.
- **`FartPlayer`** (`src/play_fart_sound.rs`) — abstracts audio playback. `RealFartPlayer` in production; `MockFartPlayer` in tests sets a `Cell<bool>` flag. A fart sound plays on every command (except `g fart` itself) when the stash is non-empty, via a daemon process spawned from the same binary (`_fart_daemon` hidden subcommand). The daemon registry lives at `/tmp/.trunk/fart_vault`.
- **`CoAuthorAliases`** (`src/co_author_aliases.rs`) — abstracts alias storage. `RealCoAuthorAliases` reads/writes `~/.config/trunk/aliases`; `InMemoryCoAuthorAliases` uses a `HashMap` for tests.

The app is assembled in `main.rs` with real implementations, then `AppService::dispatch_command` calls through `run_cli` (`src/cli.rs`) which pattern-matches on `Commands` and delegates to each command module.

### Command modules

| Command | Module | Key behavior |
|---|---|---|
| `g c` (commit) | `src/commit.rs` | Stage all, commit with co-authors in message body, then pull --rebase + push. `CommitOpt` enum splits `Message`, `Resolve`, `Abort`. |
| `g p` (pull) | `src/pull.rs` | Guarded: rejects if dirty working dir or unpushed commits. |
| `g l/s/d` | `src/query.rs` | Read-only passthroughs to `git log/status/diff`. |
| `g tt` (time travel) | `src/time_travel.rs` | Checks out a hash or resolves relative-time strings like "2 hours ago" via `git2` revwalk. `g tt now` returns to the default branch. Blocks commits/reverts while detached. |
| `g r` (reset) | `src/reset.rs` | Hard reset (HEAD → working tree). Also removes untracked files (like `git clean`). |
| `g rv` (revert) | `src/revert.rs` | Interactive confirm, then `git revert --no-edit`, then pull+push. Same resolve/abort pattern as commit. |

### Co-author commit format

`g c` requires explicit authorship — either `SOLO` or one or more `@alias` references. Solo commits made with the `SOLO` keyword include `(Solo-work)` in the message body; co-author commits include `Co-authored-by: Name <email>` lines. `@alias` is resolved from `~/.config/trunk/aliases` (format: `alias:Name <email>`, one per line). Use `g add-alias @foo -n "Name" -e "email"` to add entries.

## Testing patterns

All integration tests live in `tests/` and use `TestApp` (`tests/common/test_app.rs`), which wires the fake implementations together. Tests set up real git repos on disk via `use_git.rs` helpers (`set_up_basic_repo`, `clone_repo`, etc.) in a `TempDir`.

- **`TestApp`** builds `Cli` structs directly and calls `AppService::dispatch_command`, routing through the same `run_cli` code path as production.
- **`CapturingSink::take()`** is stateful — returns accumulated output since the last `take()`, then clears the buffer. Call it once per command to assert on its output.
- **`MockFartPlayer::was_played()`** returns a bool, reset by creating a new `TestApp`.
- **Macro-based test suite reuse** (`tests/test_co_author_aliases.rs`): `aliases_test_suite!` takes a creation macro and generates an identical `mod` of tests that run against both `RealCoAuthorAliases` (with a tempfile path) and `InMemoryCoAuthorAliases`.

## Build script

`build.rs` bakes the current short git hash into `GIT_HASH` env var, used by `version_string()` in `cli.rs` for `--version` output. Re-runs when `.git/HEAD` or refs change.

## Verbosity
Do not put comments in your code, unless it's used as a doctest in the screenplay lib.
