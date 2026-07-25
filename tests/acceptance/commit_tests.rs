use crate::abilities::{ScenarioContext, TestContext};
use crate::cast::{developer_bob, developer_kent};
use crate::interactions::{
    CloneRepo, Commit, DeleteFile, InitialCommit, Pull, SetUpRemote, WriteFile,
};
use crate::questions::{FileExists, Log, Status};
use screenplay::*;

#[test]
fn bob_commits_kent_pulls() {
    let ctx = ScenarioContext::new(TestContext::new());

    let bob = developer_bob(&ctx);
    let kent = developer_kent(&ctx);

    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "bob" },));
    bob.attempts_to((InitialCommit,));

    kent.attempts_to((CloneRepo { name: "kent" },));

    bob.attempts_to((
        WriteFile {
            name: "hello.txt",
            content: "hello world\n",
        },
        Commit {
            message: Some("add hello.txt"),
            co_authors: vec!["SOLO"],
        },
        Ensure::that(Log, contains("add hello.txt")),
    ));

    kent.attempts_to((Pull, Ensure::that(Log, contains("add hello.txt"))));

    bob.attempts_to((Ensure::that(Status, contains("nothing to commit")),));
}

#[test]
fn commit_stages_deleted_files() {
    let ctx = ScenarioContext::new(TestContext::new());

    let bob = developer_bob(&ctx);
    let kent = developer_kent(&ctx);

    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "bob" },));
    bob.attempts_to((InitialCommit,));

    kent.attempts_to((CloneRepo { name: "kent" },));

    bob.attempts_to((
        WriteFile {
            name: "to_delete.txt",
            content: "goodbye\n",
        },
        Commit {
            message: Some("add file that will be deleted"),
            co_authors: vec!["SOLO"],
        },
        DeleteFile {
            name: "to_delete.txt",
        },
        Commit {
            message: Some("delete the file"),
            co_authors: vec!["SOLO"],
        },
        Ensure::that(Log, contains("delete the file")),
        Ensure::that(
            FileExists {
                name: "to_delete.txt",
            },
            is_false(),
        ),
    ));

    kent.attempts_to((
        Pull,
        Ensure::that(
            FileExists {
                name: "to_delete.txt",
            },
            is_false(),
        ),
    ));
}

#[test]
fn missing_commit_message_is_rejected() {
    let ctx = ScenarioContext::new(TestContext::new());
    let bob = developer_bob(&ctx);

    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "dev" },));
    bob.attempts_to((InitialCommit,));

    bob.attempts_to((Ensure::that(
        doing(Commit {
            message: None,
            co_authors: vec!["SOLO"],
        }),
        fails().with_error("A commit message is required"),
    ),));
}
