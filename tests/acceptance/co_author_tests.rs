use crate::abilities::{ScenarioContext, TestContext};
use crate::cast::developer_bob;
use crate::interactions::{
    AddAlias, CloneRepo, Commit, Config, InitialCommit, SetUpRemote, WriteFile,
};
use crate::questions::Log;
use screenplay::*;

#[test]
fn committing_solo_work() {
    let ctx = ScenarioContext::new(TestContext::new());
    let bob = developer_bob(&ctx);

    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "dev" },));
    bob.attempts_to((InitialCommit,));

    bob.attempts_to((
        WriteFile {
            name: "solo.txt",
            content: "solo",
        },
        Commit {
            message: Some("solo commit"),
            co_authors: vec!["SOLO"],
        },
        Ensure::that(Log, contains("solo commit")),
        Ensure::that(Log, contains("(Solo-work)")),
        Ensure::that(Log, does_not_contain("Co-authored-by:")),
    ));
}

#[test]
fn committing_without_co_authors_is_rejected() {
    let ctx = ScenarioContext::new(TestContext::new());
    let bob = developer_bob(&ctx);

    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "dev" },));
    bob.attempts_to((InitialCommit,));

    bob.attempts_to((
        WriteFile {
            name: "fail.txt",
            content: "fail",
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
fn committing_with_a_known_alias() {
    let ctx = ScenarioContext::new(TestContext::new());
    let bob = developer_bob(&ctx);

    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "dev" },));
    bob.attempts_to((InitialCommit,));

    bob.attempts_to((
        AddAlias {
            alias: "jdoe",
            name: "John Doe",
            email: "jdoe@example.com",
        },
        WriteFile {
            name: "alias.txt",
            content: "alias",
        },
        Commit {
            message: Some("alias commit"),
            co_authors: vec!["@jdoe"],
        },
        Ensure::that(Log, contains("alias commit")),
        Ensure::that(Log, contains("Co-authored-by: John Doe <jdoe@example.com>")),
    ));
}

#[test]
fn committing_with_an_unknown_alias_is_rejected() {
    let ctx = ScenarioContext::new(TestContext::new());
    let bob = developer_bob(&ctx);

    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "dev" },));
    bob.attempts_to((InitialCommit,));

    bob.attempts_to((
        AddAlias {
            alias: "known",
            name: "Name",
            email: "email@example.com",
        },
        WriteFile {
            name: "unknown.txt",
            content: "unknown",
        },
        Ensure::that(
            doing(Commit {
                message: Some("unknown alias"),
                co_authors: vec!["@unknown"],
            }),
            fails().with_error("Unknown co-author alias: @unknown"),
        ),
    ));
}

#[test]
fn committing_with_multiple_co_authors() {
    let ctx = ScenarioContext::new(TestContext::new());
    let bob = developer_bob(&ctx);

    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "dev" },));
    bob.attempts_to((InitialCommit,));

    bob.attempts_to((
        AddAlias {
            alias: "jdoe",
            name: "John Doe",
            email: "jdoe@example.com",
        },
        AddAlias {
            alias: "asmith",
            name: "Alice Smith",
            email: "asmith@example.com",
        },
        WriteFile {
            name: "multi.txt",
            content: "multi",
        },
        Commit {
            message: Some("multi commit"),
            co_authors: vec!["@jdoe", "@asmith"],
        },
        Ensure::that(Log, contains("multi commit")),
        Ensure::that(Log, contains("Co-authored-by: John Doe <jdoe@example.com>")),
        Ensure::that(
            Log,
            contains("Co-authored-by: Alice Smith <asmith@example.com>"),
        ),
    ));
}

#[test]
fn combining_solo_with_other_co_authors_is_rejected() {
    let ctx = ScenarioContext::new(TestContext::new());
    let bob = developer_bob(&ctx);

    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "dev" },));
    bob.attempts_to((InitialCommit,));

    bob.attempts_to((
        AddAlias {
            alias: "jdoe",
            name: "John Doe",
            email: "jdoe@example.com",
        },
        WriteFile {
            name: "invalid.txt",
            content: "invalid",
        },
        Ensure::that(
            doing(Commit {
                message: Some("invalid commit"),
                co_authors: vec!["@jdoe", "SOLO"],
            }),
            fails().with_error("SOLO cannot be combined with other co-authors."),
        ),
    ));
}

#[test]
fn committing_without_co_authors_when_config_disabled() {
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
        WriteFile {
            name: "noauthor.txt",
            content: "content",
        },
        Commit {
            message: Some("commit without co-authors"),
            co_authors: vec![],
        },
        Ensure::that(Log, contains("commit without co-authors")),
        Ensure::that(Log, does_not_contain("(Solo-work)")),
    ));
}

#[test]
fn committing_with_co_authors_when_config_disabled() {
    let ctx = ScenarioContext::new(TestContext::new());
    let bob = developer_bob(&ctx);

    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "dev" },));
    bob.attempts_to((InitialCommit,));

    bob.attempts_to((
        AddAlias {
            alias: "jdoe",
            name: "John Doe",
            email: "jdoe@example.com",
        },
        Config {
            co_authors_required: Some(false),
            auto_update_period: None,
            local: false,
        },
        WriteFile {
            name: "coauthor.txt",
            content: "content",
        },
        Commit {
            message: Some("commit with co-author"),
            co_authors: vec!["@jdoe"],
        },
        Ensure::that(Log, contains("Co-authored-by: John Doe <jdoe@example.com>")),
    ));
}

#[test]
fn committing_solo_when_config_disabled() {
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
        WriteFile {
            name: "solo.txt",
            content: "solo",
        },
        Commit {
            message: Some("solo commit"),
            co_authors: vec!["SOLO"],
        },
        Ensure::that(Log, contains("solo commit")),
        Ensure::that(Log, contains("(Solo-work)")),
    ));
}
