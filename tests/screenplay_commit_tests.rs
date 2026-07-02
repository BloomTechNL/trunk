//! Screenplay-pattern collaboration tests for `g`.
//!
//! Tests that model multiple developers working together through
//! trunk-based development — committing, pulling, and checking status.

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
// Happy-path collaboration tests
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
