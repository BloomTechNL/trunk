use crate::abilities::{AccessScenarioContext, UseTrunk};
use g_cli::Commands;
use screenplay::{Ability, Actor, Interaction};

pub struct RevertHead;

impl Interaction for RevertHead {
    fn perform_as(&self, actor: &Actor) {
        let trunk = UseTrunk::by(actor);
        let asc = AccessScenarioContext::by(actor);
        let dir = &asc.actor_context(actor).working_dir;
        let log = trunk.dispatch_and_capture(Commands::Log, dir);
        let hash = log
            .lines()
            .filter(|l| l.starts_with("commit "))
            .map(|l| {
                l.strip_prefix("commit ")
                    .expect("line starts with 'commit '")
                    .to_string()
            })
            .next()
            .expect("no commit found in log");
        trunk
            .dispatch(
                Commands::Revert {
                    resolve: false,
                    abort: false,
                    noninteractive: true,
                    hash: Some(hash),
                },
                dir,
            )
            .expect("g rv should succeed");
    }
}
