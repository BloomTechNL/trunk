mod abilities;
mod cast;
mod common;
mod interactions;
mod questions;

use abilities::{ScenarioContext, TestContext};
use cast::developer_bob;
use interactions::{CloneRepo, DeleteFile, InitialCommit, SetUpRemote, WriteFile};
use questions::Diff;
use screenplay::*;

#[test]
fn diff_shows_modified_file() {
    let ctx = ScenarioContext::new(TestContext::new());
    let bob = developer_bob(&ctx);

    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "dev" },));
    bob.attempts_to((InitialCommit,));

    bob.attempts_to((
        WriteFile {
            name: "README.md",
            content: "# modified project\n",
        },
        Ensure::that(Diff, contains("README.md")),
        Ensure::that(Diff, contains("modified project")),
    ));
}

#[test]
fn diff_is_empty_when_clean() {
    let ctx = ScenarioContext::new(TestContext::new());
    let bob = developer_bob(&ctx);

    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "dev" },));
    bob.attempts_to((InitialCommit,));

    bob.attempts_to((Ensure::that(Diff, equals(String::new())),));
}

#[test]
fn diff_shows_deleted_content() {
    let ctx = ScenarioContext::new(TestContext::new());
    let bob = developer_bob(&ctx);

    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "dev" },));
    bob.attempts_to((InitialCommit,));

    bob.attempts_to((
        DeleteFile { name: "README.md" },
        Ensure::that(Diff, contains("README.md")),
    ));
}
