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

use abilities::{
    AccessScenarioContext, ScenarioContext, TestContext, UseFileSystem, UseGit, UseTrunk,
};
use interactions::{CloneRepo, Commit, InitialCommit, Pull, SetUpRemote, WriteFile};
use screenplay::*;

// ---------------------------------------------------------------------------
// Questions — ask about state
// ---------------------------------------------------------------------------

/// Ask for the output of `g l` in the actor's repo.
struct Log;

impl Question<String> for Log {
    fn answered_by(&self, actor: &Actor) -> String {
        let trunk = actor.ability::<UseTrunk>().expect("actor needs UseTrunk");
        let git = actor.ability::<UseGit>().expect("actor needs UseGit");
        trunk.app.log(&git.repo.borrow())
    }
}

/// Ask for the output of `g s` in the actor's repo.
struct Status;

impl Question<String> for Status {
    fn answered_by(&self, actor: &Actor) -> String {
        let trunk = actor.ability::<UseTrunk>().expect("actor needs UseTrunk");
        let git = actor.ability::<UseGit>().expect("actor needs UseGit");
        trunk.app.status(&git.repo.borrow())
    }
}

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
