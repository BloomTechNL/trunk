# CLAUDE.md

## Build, Test, Lint

```bash
cargo build                    # debug build
cargo build --release          # release build
cargo test -q                  # run all tests
cargo check                    # fast compile-check (no codegen)
cargo fmt -- --check           # verify formatting
cargo fmt                      # auto-format
```

## Architecture

`g` is a deliberately narrow git wrapper for trunk-based development.

### When to use `git2` vs shelling out to `git`

**Use `git2` for all local repository operations** — repo introspection, index inspection, diffing, status checks, conflict detection.

**Shell out to `git` only in two cases:**
1. **Network I/O** — push, pull, fetch, clone.
2. **Passing through output to the user** — when the user needs to see raw git output (log, status, diff display).

This means `git_passthrough` / `git_capture` (defined in `src/git.rs`) should be reserved for those two cases; new local operations should prefer `git2`.

### Trait-based dependency injection

Every side effect lives behind a trait. `AppService` is generic over those traits. Production (`main.rs`) passes real implementations; tests pass fakes. The command functions receive `&impl Trait` — they never branch on "am I in test mode?" because they don't know. The type system picks the implementation at compile time.

There are zero `#[cfg(test)]` gates in `src/`. There are zero `if testing { fake() } else { real() }` branches. The seam is the generic parameter.

The three core traits: `OutputSink` (stdout abstraction), `FartPlayer` (audio), `CoAuthorAliases` (alias storage). Each has a real implementation in `src/` and a fake in `tests/common/`.

### Co-author commit format

`g c` requires explicit authorship — either `SOLO` or one or more `@alias` references. Solo commits include `(Solo-work)` in the message body; co-author commits include `Co-authored-by: Name <email>` lines. `@alias` is resolved from `~/.config/trunk/aliases` (format: `alias:Name <email>`, one per line).

## TDD workflow

This project uses test-driven development via `/red`, `/green`, `/refactor` slash commands. Invoke them in sequence for each cycle.

## Code style

Do not put comments in your code, unless it's used as a doctest in the screenplay lib.
