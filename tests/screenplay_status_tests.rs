mod abilities;
mod cast;
mod common;
mod interactions;
mod questions;

use abilities::{ScenarioContext, TestContext};
use cast::developer_bob;
use interactions::{CloneRepo, InitialCommit, SetUpRemote, WriteFile};
use questions::Status;
use screenplay::*;

#[test]
fn status_shows_untracked_files() {
    let ctx = ScenarioContext::new(TestContext::new());
    let dev = developer_bob(&ctx);

    dev.attempts_to((SetUpRemote,));
    dev.attempts_to((CloneRepo { name: "dev" },));
    dev.attempts_to((InitialCommit,));

    dev.attempts_to((
        WriteFile {
            name: "new_file.txt",
            content: "fresh content\n",
        },
        Ensure::that(Status, contains("new_file.txt")),
        Ensure::that(Status, contains("Untracked files")),
    ));
}

#[test]
fn status_shows_clean_working_tree() {
    let ctx = ScenarioContext::new(TestContext::new());
    let dev = developer_bob(&ctx);

    dev.attempts_to((SetUpRemote,));
    dev.attempts_to((CloneRepo { name: "dev" },));
    dev.attempts_to((InitialCommit,));

    dev.attempts_to((Ensure::that(Status, contains("nothing to commit")),));
}
