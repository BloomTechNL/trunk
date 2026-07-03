use crate::abilities::{ScenarioContext, TestContext};
use crate::cast::developer_bob;
use crate::interactions::{CloneRepo, CreateDir, InitialCommit, Reset, SetUpRemote, WriteFile};
use crate::questions::{FileContent, FileExists};
use screenplay::*;

#[test]
fn reset_clears_tracked_and_untracked_changes() {
    let ctx = ScenarioContext::new(TestContext::new());
    let bob = developer_bob(&ctx);

    bob.attempts_to((SetUpRemote,));
    bob.attempts_to((CloneRepo { name: "bob" },));
    bob.attempts_to((InitialCommit,));

    bob.attempts_to((
        CreateDir { name: "subdir" },
        WriteFile {
            name: "untracked_at_root.txt",
            content: "I should disappear\n",
        },
        WriteFile {
            name: "subdir/untracked_in_subdir.txt",
            content: "also gone\n",
        },
        WriteFile {
            name: "README.md",
            content: "dirty modification\n",
        },
        Reset,
        Ensure::that(
            FileContent { name: "README.md" },
            does_not_contain("dirty modification"),
        ),
        Ensure::that(
            FileExists {
                name: "untracked_at_root.txt",
            },
            is_false(),
        ),
        Ensure::that(
            FileExists {
                name: "subdir/untracked_in_subdir.txt",
            },
            is_false(),
        ),
    ));
}
