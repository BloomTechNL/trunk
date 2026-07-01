//! Screenplay-pattern integration tests for `g`.
//!
//! These tests use the [`screenplay`] framework to model `g`'s happy path
//! through three abilities — `UseTrunk`, `UseGit`, and `UseFileSystem` — plus
//! a shared [`AccessScenarioContext`] that holds infrastructure both actors
//! need (the bare `origin.git` remote).
//!
//! Tests model multiple developers as separate [`Actor`]s, each with their
//! own repo checkout. Every git operation (including repo setup) goes through
//! actor interactions.

mod abilities;
mod common;
mod interactions;
mod questions;

use abilities::{
    AccessScenarioContext, ScenarioContext, TestContext, UseFileSystem, UseGit, UseTrunk,
};
use interactions::{CloneRepo, Commit, InitialCommit, Pull, SetUpRemote, WriteFile};
use questions::{Log, Status};
use screenplay::*;

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// Two developers collaborate through trunk-based development.
///
/// Bob creates a commit, Alice pulls and sees it. Every git operation
/// (including repo setup) is an actor interaction. The shared temp directory
/// is hidden inside [`TestContext`] — the test function only sees
/// [`ScenarioContext`] and [`AccessScenarioContext`].
#[test]
fn bob_commits_alice_pulls() {
    // -- Shared infrastructure (no Arc / TempDir visible) --------------------
    let ctx = ScenarioContext::new(TestContext::new());

    // -- Assemble the actors -------------------------------------------------
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

    // -- Arrange: repos ------------------------------------------------------
    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "bob" },));
    bob.attempts_to((InitialCommit,));

    alice.attempts_to((CloneRepo { name: "alice" },));

    // -- Act & assert: Bob writes a file and commits -------------------------
    bob.attempts_to((
        WriteFile {
            name: "hello.txt",
            content: "hello world\n",
        },
        Commit {
            message: "add hello.txt",
            co_authors: vec!["SOLO"],
        },
        Ensure::that(Log, contains("add hello.txt")),
    ));

    // -- Act & assert: Alice pulls and sees Bob's commit ---------------------
    alice.attempts_to((Pull, Ensure::that(Log, contains("add hello.txt"))));

    // -- Assert: Bob's working tree is still clean ---------------------------
    bob.attempts_to((Ensure::that(Status, contains("nothing to commit")),));
}

/// Bob clones a repo using the trunk CLI, commits, and sees his commit in
/// the logs.
///
/// This is the simplest happy-path test: set up a remote, clone it with `g
/// clone`, write a file, commit with `g c`, and verify the commit appears in
/// `g l`.
#[test]
fn bob_can_clone_a_repo() {
    // -- Shared infrastructure ------------------------------------------------
    let ctx = ScenarioContext::new(TestContext::new());

    // -- Assemble the actor ---------------------------------------------------
    let bob = Actor::new()
        .who_can(AccessScenarioContext::new(&ctx))
        .who_can(UseTrunk::new())
        .who_can(UseGit::new())
        .who_can(UseFileSystem);

    // -- Arrange: repos -------------------------------------------------------
    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "bob" },));
    bob.attempts_to((InitialCommit,));

    // -- Act & assert: Bob writes a file and commits --------------------------
    bob.attempts_to((
        WriteFile {
            name: "hello.txt",
            content: "hello from bob\n",
        },
        Commit {
            message: "add hello.txt",
            co_authors: vec!["SOLO"],
        },
        Ensure::that(Log, contains("add hello.txt")),
    ));
}
