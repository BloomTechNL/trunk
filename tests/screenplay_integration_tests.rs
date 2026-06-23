//! Screenplay-pattern integration tests for `g`.
//!
//! These tests use the [`screenplay`] framework to model `g`ʼs happy path
//! through three abilities — `UseTrunk`, `UseGit`, and `UseFileSystem` — each
//! representing a capability an actor can draw on.
//!
//! Tests model multiple developers as separate [`Actor`]s, each with their
//! own repo checkout.

mod common;

use std::path::PathBuf;

use common::test_app::TestApp;
use common::use_git::{clone_repo, initial_commit, set_up_remote};
use common::write_file::write_file;
use screenplay::*;
use tempfile::TempDir;

// ---------------------------------------------------------------------------
// Abilities
// ---------------------------------------------------------------------------

/// Ability to dispatch `g` subcommands (`g c`, `g p`, `g l`, `g s`, …).
struct UseTrunk {
    app: TestApp,
}

impl Ability for UseTrunk {}

impl UseTrunk {
    fn new() -> Self {
        UseTrunk {
            app: TestApp::new(),
        }
    }
}

/// Ability to operate on a local git checkout.
///
/// Each actor gets their own `UseGit` pointed at their personal clone, so
/// interactions like [`Commit`] or [`Pull`] automatically target the right
/// repo without the caller having to specify a path.
struct UseGit {
    repo: PathBuf,
}

impl Ability for UseGit {}

impl UseGit {
    fn new(repo: PathBuf) -> Self {
        UseGit { repo }
    }
}

/// Marker ability — signals the actor is allowed to touch the file system.
struct UseFileSystem;

impl Ability for UseFileSystem {}

// ---------------------------------------------------------------------------
// Interactions
// ---------------------------------------------------------------------------

/// Write a file into the actor's repo.
struct WriteFile {
    name: &'static str,
    content: &'static str,
}

impl Interaction for WriteFile {
    fn perform_as(&self, actor: &Actor) {
        let _fs = actor
            .ability::<UseFileSystem>()
            .expect("actor needs UseFileSystem");
        let git = actor.ability::<UseGit>().expect("actor needs UseGit");
        write_file(&git.repo, self.name, self.content);
    }
}

/// Run `g c` in the actor's repo.
struct Commit {
    message: &'static str,
    co_authors: Vec<&'static str>,
}

impl Interaction for Commit {
    fn perform_as(&self, actor: &Actor) {
        let trunk = actor.ability::<UseTrunk>().expect("actor needs UseTrunk");
        let git = actor.ability::<UseGit>().expect("actor needs UseGit");
        trunk
            .app
            .commit(
                &git.repo,
                self.message,
                self.co_authors.iter().map(|s| *s).collect(),
            )
            .expect("g c should succeed");
    }
}

/// Run `g p` in the actor's repo.
struct Pull;

impl Interaction for Pull {
    fn perform_as(&self, actor: &Actor) {
        let trunk = actor.ability::<UseTrunk>().expect("actor needs UseTrunk");
        let git = actor.ability::<UseGit>().expect("actor needs UseGit");
        trunk.app.pull(&git.repo).expect("g p should succeed");
    }
}

// ---------------------------------------------------------------------------
// Questions
// ---------------------------------------------------------------------------

/// Ask for the output of `g l` in the actor's repo.
struct Log;

impl Question<String> for Log {
    fn answered_by(&self, actor: &Actor) -> String {
        let trunk = actor.ability::<UseTrunk>().expect("actor needs UseTrunk");
        let git = actor.ability::<UseGit>().expect("actor needs UseGit");
        trunk.app.log(&git.repo)
    }
}

/// Ask for the output of `g s` in the actor's repo.
struct Status;

impl Question<String> for Status {
    fn answered_by(&self, actor: &Actor) -> String {
        let trunk = actor.ability::<UseTrunk>().expect("actor needs UseTrunk");
        let git = actor.ability::<UseGit>().expect("actor needs UseGit");
        trunk.app.status(&git.repo)
    }
}

// ---------------------------------------------------------------------------
// Expectations
// ---------------------------------------------------------------------------

/// Expect that a `String` contains the given substring.
struct Contains {
    expected: String,
}

impl Expectation<String> for Contains {
    fn test(&self, value: &String) -> bool {
        value.contains(&self.expected)
    }

    fn message(&self, value: &String) -> String {
        format!(
            "Expected output to contain {:?}, but got:\n{}",
            self.expected, value
        )
    }
}

/// Construct a [`Contains`] expectation.
fn contains(expected: impl Into<String>) -> impl Expectation<String> {
    Contains {
        expected: expected.into(),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// Two developers collaborate through trunk-based development.
///
/// Bob creates a commit, Alice pulls and sees it. This is the screenplay
/// equivalent of `test_clean_commit_flow`.
#[test]
fn bob_commits_alice_pulls() {
    // -- Arrange: shared remote infrastructure ------------------------------
    let base = TempDir::new().expect("temp dir");
    set_up_remote(base.path());

    let bob_repo = clone_repo(base.path(), "bob", "origin.git");
    initial_commit(&bob_repo);

    let alice_repo = clone_repo(base.path(), "alice", "origin.git");

    // -- Assemble the actors ------------------------------------------------
    let bob = Actor::new()
        .who_can(UseTrunk::new())
        .who_can(UseGit::new(bob_repo))
        .who_can(UseFileSystem);

    let alice = Actor::new()
        .who_can(UseTrunk::new())
        .who_can(UseGit::new(alice_repo))
        .who_can(UseFileSystem);

    // -- Bob: write a file and commit ---------------------------------------
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

    // -- Alice: pull and see Bob's commit -----------------------------------
    alice.attempts_to((Pull, Ensure::that(Log, contains("add hello.txt"))));

    // -- Bob: working tree is still clean -----------------------------------
    bob.attempts_to((Ensure::that(Status, contains("nothing to commit")),));
}
