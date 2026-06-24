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

mod common;

use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use common::test_app::TestApp;
use common::use_git::{clone_repo, initial_commit, set_up_remote};
use common::write_file::write_file;
use screenplay::*;
use tempfile::TempDir;

// ---------------------------------------------------------------------------
// Scenario context — shared test infrastructure (client-side, not in framework)
// ---------------------------------------------------------------------------

/// Shared state that every actor in the scenario can access.
///
/// Owns the temp directory where `origin.git` lives so both developers can
/// clone from the same remote.
struct TestContext {
    base_dir: TempDir,
}

impl TestContext {
    fn new() -> Self {
        TestContext {
            base_dir: TempDir::new().expect("temp dir"),
        }
    }
}

/// A reference-counted, interior-mutable handle to the shared [`TestContext`].
///
/// Created once by the test function. Each actor that needs shared state
/// receives an [`AccessScenarioContext`] ability cloned from this.
struct ScenarioContext {
    inner: Rc<RefCell<TestContext>>,
}

impl ScenarioContext {
    fn new(ctx: TestContext) -> Self {
        ScenarioContext {
            inner: Rc::new(RefCell::new(ctx)),
        }
    }
}

/// Ability that gives an [`Actor`] access to the shared [`ScenarioContext`].
///
/// The inner `Rc<RefCell<TestContext>>` is deliberately exposed — borrow it
/// with `.context.borrow()` or `.context.borrow_mut()` inside interactions
/// and questions.
struct AccessScenarioContext {
    context: Rc<RefCell<TestContext>>,
}

impl Ability for AccessScenarioContext {}

impl AccessScenarioContext {
    fn new(ctx: &ScenarioContext) -> Self {
        AccessScenarioContext {
            context: ctx.inner.clone(),
        }
    }
}

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
/// Each actor's `UseGit` holds the path to their own repo clone. The repo
/// path is set during the arrange phase by [`CloneRepo`]; later interactions
/// like [`Commit`] or [`Pull`] read it automatically.
///
/// The shared base directory (where `origin.git` lives) is stored in
/// [`AccessScenarioContext`], not here.
struct UseGit {
    repo: RefCell<PathBuf>,
}

impl Ability for UseGit {}

impl UseGit {
    fn new() -> Self {
        UseGit {
            repo: RefCell::new(PathBuf::new()),
        }
    }
}

/// Marker ability — signals the actor is allowed to touch the file system.
struct UseFileSystem;

impl Ability for UseFileSystem {}

// ---------------------------------------------------------------------------
// Interactions — arrange / act
// ---------------------------------------------------------------------------

/// Create a bare `origin.git` remote inside the shared base directory.
struct SetUpRemote;

impl Interaction for SetUpRemote {
    fn perform_as(&self, actor: &Actor) {
        let ctx = actor
            .ability::<AccessScenarioContext>()
            .expect("actor needs AccessScenarioContext");
        let _fs = actor
            .ability::<UseFileSystem>()
            .expect("actor needs UseFileSystem");
        set_up_remote(ctx.context.borrow().base_dir.path());
    }
}

/// Clone from `origin.git` into `base_dir/<name>` and record the path as
/// this actor's repo.
struct CloneRepo {
    name: &'static str,
}

impl Interaction for CloneRepo {
    fn perform_as(&self, actor: &Actor) {
        let ctx = actor
            .ability::<AccessScenarioContext>()
            .expect("actor needs AccessScenarioContext");
        let _fs = actor
            .ability::<UseFileSystem>()
            .expect("actor needs UseFileSystem");
        let git = actor.ability::<UseGit>().expect("actor needs UseGit");
        let path = clone_repo(
            ctx.context.borrow().base_dir.path(),
            self.name,
            "origin.git",
        );
        *git.repo.borrow_mut() = path;
    }
}

/// Make an initial commit (README) and push to origin so other actors can
/// see it when they clone.
struct InitialCommit;

impl Interaction for InitialCommit {
    fn perform_as(&self, actor: &Actor) {
        let _fs = actor
            .ability::<UseFileSystem>()
            .expect("actor needs UseFileSystem");
        let git = actor.ability::<UseGit>().expect("actor needs UseGit");
        initial_commit(&git.repo.borrow());
    }
}

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
        write_file(&git.repo.borrow(), self.name, self.content);
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
                &git.repo.borrow(),
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
        trunk
            .app
            .pull(&git.repo.borrow())
            .expect("g p should succeed");
    }
}

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
