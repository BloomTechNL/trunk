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
    let bob = developer_bob(&ctx);

    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "dev" },));
    bob.attempts_to((InitialCommit,));

    bob.attempts_to((
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
    let bob = developer_bob(&ctx);

    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "dev" },));
    bob.attempts_to((InitialCommit,));

    bob.attempts_to((Ensure::that(Status, contains("nothing to commit")),));
}
