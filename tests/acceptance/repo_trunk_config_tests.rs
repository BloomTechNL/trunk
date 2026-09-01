use crate::abilities::{ScenarioContext, TestContext};
use crate::cast::developer_bob;
use crate::interactions::{CloneRepo, Commit, Config, InitialCommit, SetUpRemote, WriteFile};
use crate::questions::Log;
use screenplay::*;

#[test]
fn repo_local_config_relaxes_co_author_requirement() {
    let ctx = ScenarioContext::new(TestContext::new());
    let bob = developer_bob(&ctx);

    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "dev" },));
    bob.attempts_to((InitialCommit,));

    bob.attempts_to((
        WriteFile {
            name: ".trunk.json",
            content: r#"{"coAuthorsRequired": false}"#,
        },
        WriteFile {
            name: "noauthor.txt",
            content: "content",
        },
        Commit {
            message: Some("commit without co-authors"),
            co_authors: vec![],
        },
        Ensure::that(Log, contains("commit without co-authors")),
    ));
}

#[test]
fn repo_local_config_overrides_global_in_favor_of_stricter_rule() {
    let ctx = ScenarioContext::new(TestContext::new());
    let bob = developer_bob(&ctx);

    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "dev" },));
    bob.attempts_to((InitialCommit,));

    bob.attempts_to((
        Config {
            co_authors_required: Some(false),
            auto_update_period: None,
        },
        WriteFile {
            name: ".trunk.json",
            content: r#"{"coAuthorsRequired": true}"#,
        },
        WriteFile {
            name: "noauthor.txt",
            content: "content",
        },
        Ensure::that(
            doing(Commit {
                message: Some("no authors"),
                co_authors: vec![],
            }),
            fails().with_error(
                "You must either specify co-authors as @jane @john or specify that this is solo work with SOLO",
            ),
        ),
    ));
}
