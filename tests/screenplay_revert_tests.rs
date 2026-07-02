mod abilities;
mod cast;
mod common;
mod interactions;
mod questions;

use abilities::{ScenarioContext, TestContext};
use cast::{developer_bob, developer_kent};
use interactions::{CloneRepo, Commit, InitialCommit, RevertHead, SetUpRemote, WriteFile};
use questions::{FileExists, Log};
use screenplay::*;

#[test]
fn reverting_a_commit() {
    let ctx = ScenarioContext::new(TestContext::new());
    let bob = developer_bob(&ctx);

    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "bob" },));
    bob.attempts_to((InitialCommit,));

    bob.attempts_to((
        WriteFile {
            name: "to_revert.txt",
            content: "this will be reverted\n",
        },
        Commit {
            message: "add file to revert",
            co_authors: vec!["SOLO"],
        },
        RevertHead,
        Ensure::that(Log, contains("Revert")),
        Ensure::that(
            FileExists {
                name: "to_revert.txt",
            },
            is_false(),
        ),
    ));
}

#[test]
fn reverting_without_a_remote_tracking_branch() {
    let ctx = ScenarioContext::new(TestContext::new());

    let bob = developer_bob(&ctx);
    let kent = developer_kent(&ctx);

    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "clone" },));
    bob.attempts_to((InitialCommit,));

    kent.attempts_to((CloneRepo { name: "clone2" },));

    bob.attempts_to((
        WriteFile {
            name: "a.txt",
            content: "a\n",
        },
        Commit {
            message: "add a",
            co_authors: vec!["SOLO"],
        },
    ));

    kent.attempts_to((
        WriteFile {
            name: "b.txt",
            content: "b\n",
        },
        Commit {
            message: "add b",
            co_authors: vec!["SOLO"],
        },
        RevertHead,
        Ensure::that(Log, contains("Revert")),
    ));
}
