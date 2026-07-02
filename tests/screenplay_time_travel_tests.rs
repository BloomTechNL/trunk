mod abilities;
mod cast;
mod common;
mod interactions;
mod questions;

use abilities::{ScenarioContext, TestContext};
use cast::developer_bob;
use interactions::{
    CloneRepo, Commit, InitialCommit, RevertHead, SetUpRemote, TimeTravel, WriteFile,
};
use questions::Log;
use screenplay::*;

#[test]
fn time_travel_blocks_writes_and_now_restores() {
    let ctx = ScenarioContext::new(TestContext::new());
    let bob = developer_bob(&ctx);

    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "bob" },));
    bob.attempts_to((InitialCommit,));

    bob.attempts_to((
        WriteFile {
            name: "v1.txt",
            content: "v1\n",
        },
        Commit {
            message: "v1",
            co_authors: vec!["SOLO"],
        },
    ));

    bob.attempts_to((
        WriteFile {
            name: "v2.txt",
            content: "v2\n",
        },
        Commit {
            message: "v2",
            co_authors: vec!["SOLO"],
        },
        TimeTravel { target: "HEAD~1" },
        WriteFile {
            name: "should_fail.txt",
            content: "nope\n",
        },
        Ensure::that(
            doing(Commit {
                message: "this should be blocked",
                co_authors: vec!["SOLO"],
            }),
            fails().with_error("time travelling"),
        ),
        Ensure::that(doing(RevertHead), fails().with_error("time travelling")),
    ));

    bob.attempts_to((
        TimeTravel { target: "now" },
        WriteFile {
            name: "after_return.txt",
            content: "back\n",
        },
        Commit {
            message: "commit after returning from time travel",
            co_authors: vec!["SOLO"],
        },
        Ensure::that(Log, contains("commit after returning from time travel")),
    ));
}
