use std::fs;
use std::os::unix::fs::PermissionsExt;

use crate::abilities::AccessScenarioContext;
use screenplay::{Ability, Actor, Interaction};

pub struct InstallPreCommitHook {
    pub exit_code: u8,
}

impl Interaction for InstallPreCommitHook {
    fn perform_as(&self, actor: &Actor) {
        let asc = AccessScenarioContext::by(actor);
        let hooks = asc.actor_context(actor).working_dir.join(".git/hooks");
        fs::create_dir_all(&hooks).expect("create hooks dir");
        let hook = hooks.join("pre-commit");
        fs::write(
            &hook,
            format!("#!/usr/bin/env bash\nexit {}\n", self.exit_code),
        )
        .expect("write pre-commit hook");
        fs::set_permissions(&hook, fs::Permissions::from_mode(0o755)).expect("chmod hook");
    }
}
