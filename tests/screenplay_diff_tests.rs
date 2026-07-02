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
    let dev = developer_bob(&ctx);

    dev.attempts_to((SetUpRemote,));
    dev.attempts_to((CloneRepo { name: "dev" },));
    dev.attempts_to((InitialCommit,));

    dev.attempts_to((
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
    let dev = developer_bob(&ctx);

    dev.attempts_to((SetUpRemote,));
    dev.attempts_to((CloneRepo { name: "dev" },));
    dev.attempts_to((InitialCommit,));

    dev.attempts_to((Ensure::that(Diff, does_not_contain("diff --git")),));
}

#[test]
fn diff_shows_deleted_content() {
    let ctx = ScenarioContext::new(TestContext::new());
    let dev = developer_bob(&ctx);

    dev.attempts_to((SetUpRemote,));
    dev.attempts_to((CloneRepo { name: "dev" },));
    dev.attempts_to((InitialCommit,));

    dev.attempts_to((
        DeleteFile { name: "README.md" },
        Ensure::that(Diff, contains("README.md")),
    ));
}
