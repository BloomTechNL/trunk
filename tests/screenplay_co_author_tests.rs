//! Screenplay-pattern co-author tests for `g`.
//!
//! Tests for co-author alias resolution, SOLO commits, config-driven
//! co-author requirements, and error handling around invalid co-author
//! combinations.

mod abilities;
mod common;
mod interactions;
mod questions;

use abilities::{
    AccessScenarioContext, ScenarioContext, TestContext, UseFileSystem, UseGit, UseTrunk,
};
use interactions::{
    AddAlias, AttemptCommit, CloneRepo, Commit, Config, InitialCommit, SetUpRemote, WriteFile,
};
use questions::{CommitError, Log};
use screenplay::*;

// ---------------------------------------------------------------------------
// Co-author tests
// ---------------------------------------------------------------------------

/// A developer commits solo work — the log records `(Solo-work)` and no
/// co-author trailer.
#[test]
fn committing_solo_work() {
    let ctx = ScenarioContext::new(TestContext::new());
    let dev = Actor::new()
        .who_can(AccessScenarioContext::new(&ctx))
        .who_can(UseTrunk::new())
        .who_can(UseGit::new())
        .who_can(UseFileSystem);

    dev.attempts_to((SetUpRemote,));
    dev.attempts_to((CloneRepo { name: "dev" },));
    dev.attempts_to((InitialCommit,));

    dev.attempts_to((
        WriteFile {
            name: "solo.txt",
            content: "solo",
        },
        Commit {
            message: "solo commit",
            co_authors: vec!["SOLO"],
        },
        Ensure::that(Log, contains("solo commit")),
        Ensure::that(Log, contains("(Solo-work)")),
        Ensure::that(Log, does_not_contain("Co-authored-by:")),
    ));
}

/// Committing with an empty co-author list is rejected with a helpful
/// message when `coAuthorsRequired` is enabled (the default).
#[test]
fn committing_without_co_authors_is_rejected() {
    let ctx = ScenarioContext::new(TestContext::new());
    let dev = Actor::new()
        .who_can(AccessScenarioContext::new(&ctx))
        .who_can(UseTrunk::new())
        .who_can(UseGit::new())
        .who_can(UseFileSystem);

    dev.attempts_to((SetUpRemote,));
    dev.attempts_to((CloneRepo { name: "dev" },));
    dev.attempts_to((InitialCommit,));

    dev.attempts_to((
        WriteFile {
            name: "fail.txt",
            content: "fail",
        },
        AttemptCommit {
            message: "no authors",
            co_authors: vec![],
        },
        Ensure::that(
            CommitError,
            contains(
                "You must either specify co-authors as @jane @john or specify that this is solo work with SOLO",
            ),
        ),
    ));
}

/// A developer adds a co-author alias and commits with it — the log includes
/// the `Co-authored-by:` trailer.
#[test]
fn committing_with_a_known_alias() {
    let ctx = ScenarioContext::new(TestContext::new());
    let dev = Actor::new()
        .who_can(AccessScenarioContext::new(&ctx))
        .who_can(UseTrunk::new())
        .who_can(UseGit::new())
        .who_can(UseFileSystem);

    dev.attempts_to((SetUpRemote,));
    dev.attempts_to((CloneRepo { name: "dev" },));
    dev.attempts_to((InitialCommit,));

    dev.attempts_to((
        AddAlias {
            alias: "jdoe",
            name: "John Doe",
            email: "jdoe@example.com",
        },
        WriteFile {
            name: "alias.txt",
            content: "alias",
        },
        Commit {
            message: "alias commit",
            co_authors: vec!["@jdoe"],
        },
        Ensure::that(Log, contains("alias commit")),
        Ensure::that(Log, contains("Co-authored-by: John Doe <jdoe@example.com>")),
    ));
}

/// Committing with an unknown alias fails and the error message suggests
/// adding the alias.
#[test]
fn committing_with_an_unknown_alias_is_rejected() {
    let ctx = ScenarioContext::new(TestContext::new());
    let dev = Actor::new()
        .who_can(AccessScenarioContext::new(&ctx))
        .who_can(UseTrunk::new())
        .who_can(UseGit::new())
        .who_can(UseFileSystem);

    dev.attempts_to((SetUpRemote,));
    dev.attempts_to((CloneRepo { name: "dev" },));
    dev.attempts_to((InitialCommit,));

    dev.attempts_to((
        AddAlias {
            alias: "known",
            name: "Name",
            email: "email@example.com",
        },
        WriteFile {
            name: "unknown.txt",
            content: "unknown",
        },
        AttemptCommit {
            message: "unknown alias",
            co_authors: vec!["@unknown"],
        },
        Ensure::that(CommitError, contains("Unknown co-author alias: @unknown")),
        Ensure::that(CommitError, contains("Please add it to")),
    ));
}

/// A developer can commit with multiple co-authors — each gets a
/// `Co-authored-by:` trailer.
#[test]
fn committing_with_multiple_co_authors() {
    let ctx = ScenarioContext::new(TestContext::new());
    let dev = Actor::new()
        .who_can(AccessScenarioContext::new(&ctx))
        .who_can(UseTrunk::new())
        .who_can(UseGit::new())
        .who_can(UseFileSystem);

    dev.attempts_to((SetUpRemote,));
    dev.attempts_to((CloneRepo { name: "dev" },));
    dev.attempts_to((InitialCommit,));

    dev.attempts_to((
        AddAlias {
            alias: "jdoe",
            name: "John Doe",
            email: "jdoe@example.com",
        },
        AddAlias {
            alias: "asmith",
            name: "Alice Smith",
            email: "asmith@example.com",
        },
        WriteFile {
            name: "multi.txt",
            content: "multi",
        },
        Commit {
            message: "multi commit",
            co_authors: vec!["@jdoe", "@asmith"],
        },
        Ensure::that(Log, contains("multi commit")),
        Ensure::that(Log, contains("Co-authored-by: John Doe <jdoe@example.com>")),
        Ensure::that(
            Log,
            contains("Co-authored-by: Alice Smith <asmith@example.com>"),
        ),
    ));
}

/// Using `SOLO` together with other co-authors is rejected.
#[test]
fn combining_solo_with_other_co_authors_is_rejected() {
    let ctx = ScenarioContext::new(TestContext::new());
    let dev = Actor::new()
        .who_can(AccessScenarioContext::new(&ctx))
        .who_can(UseTrunk::new())
        .who_can(UseGit::new())
        .who_can(UseFileSystem);

    dev.attempts_to((SetUpRemote,));
    dev.attempts_to((CloneRepo { name: "dev" },));
    dev.attempts_to((InitialCommit,));

    dev.attempts_to((
        AddAlias {
            alias: "jdoe",
            name: "John Doe",
            email: "jdoe@example.com",
        },
        WriteFile {
            name: "invalid.txt",
            content: "invalid",
        },
        AttemptCommit {
            message: "invalid commit",
            co_authors: vec!["@jdoe", "SOLO"],
        },
        Ensure::that(
            CommitError,
            contains("SOLO cannot be combined with other co-authors."),
        ),
    ));
}

/// When `coAuthorsRequired` is disabled, a commit with no co-authors
/// succeeds without the `(Solo-work)` marker.
#[test]
fn committing_without_co_authors_when_config_disabled() {
    let ctx = ScenarioContext::new(TestContext::new());
    let dev = Actor::new()
        .who_can(AccessScenarioContext::new(&ctx))
        .who_can(UseTrunk::new())
        .who_can(UseGit::new())
        .who_can(UseFileSystem);

    dev.attempts_to((SetUpRemote,));
    dev.attempts_to((CloneRepo { name: "dev" },));
    dev.attempts_to((InitialCommit,));

    dev.attempts_to((
        Config {
            co_authors_required: Some(false),
        },
        WriteFile {
            name: "noauthor.txt",
            content: "content",
        },
        Commit {
            message: "commit without co-authors",
            co_authors: vec![],
        },
        Ensure::that(Log, contains("commit without co-authors")),
        Ensure::that(Log, does_not_contain("(Solo-work)")),
    ));
}

/// Even when `coAuthorsRequired` is disabled, co-author aliases are still
/// honoured and recorded in the log.
#[test]
fn committing_with_co_authors_when_config_disabled() {
    let ctx = ScenarioContext::new(TestContext::new());
    let dev = Actor::new()
        .who_can(AccessScenarioContext::new(&ctx))
        .who_can(UseTrunk::new())
        .who_can(UseGit::new())
        .who_can(UseFileSystem);

    dev.attempts_to((SetUpRemote,));
    dev.attempts_to((CloneRepo { name: "dev" },));
    dev.attempts_to((InitialCommit,));

    dev.attempts_to((
        AddAlias {
            alias: "jdoe",
            name: "John Doe",
            email: "jdoe@example.com",
        },
        Config {
            co_authors_required: Some(false),
        },
        WriteFile {
            name: "coauthor.txt",
            content: "content",
        },
        Commit {
            message: "commit with co-author",
            co_authors: vec!["@jdoe"],
        },
        Ensure::that(Log, contains("Co-authored-by: John Doe <jdoe@example.com>")),
    ));
}

/// The `SOLO` keyword still works when `coAuthorsRequired` is disabled.
#[test]
fn committing_solo_when_config_disabled() {
    let ctx = ScenarioContext::new(TestContext::new());
    let dev = Actor::new()
        .who_can(AccessScenarioContext::new(&ctx))
        .who_can(UseTrunk::new())
        .who_can(UseGit::new())
        .who_can(UseFileSystem);

    dev.attempts_to((SetUpRemote,));
    dev.attempts_to((CloneRepo { name: "dev" },));
    dev.attempts_to((InitialCommit,));

    dev.attempts_to((
        Config {
            co_authors_required: Some(false),
        },
        WriteFile {
            name: "solo.txt",
            content: "solo",
        },
        Commit {
            message: "solo commit",
            co_authors: vec!["SOLO"],
        },
        Ensure::that(Log, contains("solo commit")),
        Ensure::that(Log, contains("(Solo-work)")),
    ));
}
