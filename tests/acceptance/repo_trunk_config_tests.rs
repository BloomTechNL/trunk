use crate::abilities::{ScenarioContext, TestContext};
use crate::cast::developer_bob;
use crate::interactions::{
    CloneRepo, Commit, Config, InitialCommit, SetUpRemote, WorkOutsideRepo, WriteFile,
};
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
        Config {
            co_authors_required: Some(false),
            auto_update_period: None,
            local: true,
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
            local: false,
        },
        Config {
            co_authors_required: Some(true),
            auto_update_period: None,
            local: true,
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

#[test]
fn setting_local_config_only_touches_the_field_that_was_set() {
    let ctx = ScenarioContext::new(TestContext::new());
    let bob = developer_bob(&ctx);

    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "dev" },));
    bob.attempts_to((InitialCommit,));

    bob.attempts_to((
        Config {
            co_authors_required: Some(false),
            auto_update_period: None,
            local: true,
        },
        Config {
            auto_update_period: Some(3_600),
            co_authors_required: None,
            local: true,
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
fn setting_local_config_outside_a_git_repository_is_rejected() {
    let ctx = ScenarioContext::new(TestContext::new());
    let bob = developer_bob(&ctx);

    bob.attempts_to((WorkOutsideRepo,));

    bob.attempts_to((Ensure::that(
        doing(Config {
            co_authors_required: Some(false),
            auto_update_period: None,
            local: true,
        }),
        fails().with_error("Not inside a git repository"),
    ),));
}
