mod abilities;
mod cast;
mod common;
mod interactions;
mod questions;

use abilities::{ScenarioContext, TestContext};
use cast::{developer_bob, developer_kent};
use interactions::{
    CloneRepo, Commit, CommitUnpushed, InitialCommit, Pull, SetUpRemote, WriteFile,
};
use questions::Log;
use screenplay::*;

#[test]
fn pull_blocked_by_unpushed_commits() {
    let ctx = ScenarioContext::new(TestContext::new());
    let bob = developer_bob(&ctx);

    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "dev" },));
    bob.attempts_to((InitialCommit,));

    bob.attempts_to((
        CommitUnpushed,
        Ensure::that(doing(Pull), fails().with_error("unpushed")),
    ));
}

#[test]
fn pull_blocked_by_dirty_working_dir() {
    let ctx = ScenarioContext::new(TestContext::new());
    let bob = developer_bob(&ctx);

    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "dev" },));
    bob.attempts_to((InitialCommit,));

    bob.attempts_to((
        WriteFile {
            name: "dirty.txt",
            content: "not yet committed\n",
        },
        Ensure::that(doing(Pull), fails().with_error("uncommitted")),
    ));
}

#[test]
fn pull_succeeds_when_clean() {
    let ctx = ScenarioContext::new(TestContext::new());

    let bob = developer_bob(&ctx);
    let kent = developer_kent(&ctx);

    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "bob" },));
    bob.attempts_to((InitialCommit,));

    kent.attempts_to((CloneRepo { name: "kent" },));

    bob.attempts_to((
        WriteFile {
            name: "new_feature.txt",
            content: "feature\n",
        },
        Commit {
            message: "add feature",
            co_authors: vec!["SOLO"],
        },
    ));

    kent.attempts_to((Pull, Ensure::that(Log, contains("add feature"))));
}
