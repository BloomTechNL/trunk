mod abilities;
mod cast;
mod common;
mod interactions;
mod questions;

use abilities::{ScenarioContext, TestContext};
use cast::{developer_bob, developer_kent};
use interactions::{CloneRepo, Commit, InitialCommit, Pull, SetUpRemote, WriteFile};
use questions::{Log, Status};
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
            message: "add hello.txt",
            co_authors: vec!["SOLO"],
        },
        Ensure::that(Log, contains("add hello.txt")),
    ));

    kent.attempts_to((Pull, Ensure::that(Log, contains("add hello.txt"))));

    bob.attempts_to((Ensure::that(Status, contains("nothing to commit")),));
}
