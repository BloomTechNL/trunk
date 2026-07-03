use crate::abilities::{ScenarioContext, TestContext};
use crate::cast::{developer_bob, developer_kent};
use crate::interactions::{
    AbortCommit, CloneRepo, Commit, InitialCommit, Pull, ResolveCommit, SetUpRemote, WriteFile,
};
use crate::questions::{Log, Status};
use screenplay::*;

#[test]
fn resolving_a_merge_conflict() {
    let ctx = ScenarioContext::new(TestContext::new());

    let bob = developer_bob(&ctx);
    let kent = developer_kent(&ctx);

    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "bob" },));
    bob.attempts_to((InitialCommit,));

    kent.attempts_to((CloneRepo { name: "kent" },));

    bob.attempts_to((
        WriteFile {
            name: "shared.txt",
            content: "version A\n",
        },
        Commit {
            message: "clone_a: add shared",
            co_authors: vec!["SOLO"],
        },
    ));

    kent.attempts_to((Pull,));

    kent.attempts_to((WriteFile {
        name: "shared.txt",
        content: "version B\n",
    },));

    bob.attempts_to((
        WriteFile {
            name: "shared.txt",
            content: "version A2\n",
        },
        Commit {
            message: "clone_a: update shared",
            co_authors: vec!["SOLO"],
        },
    ));

    kent.attempts_to((
        Ensure::that(
            doing(Commit {
                message: "clone_b: conflicting change",
                co_authors: vec!["SOLO"],
            }),
            fails(),
        ),
        WriteFile {
            name: "shared.txt",
            content: "resolved content\n",
        },
        ResolveCommit,
        Ensure::that(Log, contains("clone_b: conflicting change")),
    ));
}

#[test]
fn aborting_a_merge_conflict() {
    let ctx = ScenarioContext::new(TestContext::new());

    let bob = developer_bob(&ctx);
    let kent = developer_kent(&ctx);

    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "bob" },));
    bob.attempts_to((InitialCommit,));

    kent.attempts_to((CloneRepo { name: "kent" },));

    bob.attempts_to((
        WriteFile {
            name: "conflict.txt",
            content: "original\n",
        },
        Commit {
            message: "seed conflict file",
            co_authors: vec!["SOLO"],
        },
    ));

    kent.attempts_to((Pull,));

    bob.attempts_to((
        WriteFile {
            name: "conflict.txt",
            content: "clone_a update\n",
        },
        Commit {
            message: "clone_a update",
            co_authors: vec!["SOLO"],
        },
    ));

    kent.attempts_to((
        WriteFile {
            name: "conflict.txt",
            content: "clone_b update\n",
        },
        Ensure::that(
            doing(Commit {
                message: "clone_b conflicting",
                co_authors: vec!["SOLO"],
            }),
            fails(),
        ),
        AbortCommit,
        Ensure::that(Status, does_not_contain("nothing to commit")),
    ));
}

#[test]
fn commit_is_blocked_while_in_conflict_state() {
    let ctx = ScenarioContext::new(TestContext::new());

    let bob = developer_bob(&ctx);
    let kent = developer_kent(&ctx);

    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "bob" },));
    bob.attempts_to((InitialCommit,));

    kent.attempts_to((CloneRepo { name: "kent" },));

    bob.attempts_to((
        WriteFile {
            name: "clash.txt",
            content: "A\n",
        },
        Commit {
            message: "A init",
            co_authors: vec!["SOLO"],
        },
    ));

    kent.attempts_to((Pull,));

    bob.attempts_to((
        WriteFile {
            name: "clash.txt",
            content: "A updated\n",
        },
        Commit {
            message: "A update",
            co_authors: vec!["SOLO"],
        },
    ));

    kent.attempts_to((
        WriteFile {
            name: "clash.txt",
            content: "B update\n",
        },
        Ensure::that(
            doing(Commit {
                message: "B conflicting",
                co_authors: vec!["SOLO"],
            }),
            fails(),
        ),
        Ensure::that(
            doing(Commit {
                message: "should be blocked",
                co_authors: vec!["SOLO"],
            }),
            fails().with_error("middle of resolving a conflict"),
        ),
    ));
}
