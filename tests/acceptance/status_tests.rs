use crate::abilities::{ScenarioContext, TestContext};
use crate::cast::developer_bob;
use crate::interactions::{CloneRepo, InitialCommit, SetUpRemote, WriteFile};
use crate::questions::Status;
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
