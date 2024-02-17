use clap::ArgMatches;
use vanadin_tasks::runner::TaskRunner;

use crate::vanadir::VanadinConfig;

pub fn execute(matches: &ArgMatches) {
    let task_id: &String = matches.get_one("task").expect("task is required");

    let config = VanadinConfig::fetch();

    let task = config
        .tasks
        .iter()
        .find(|t| t.id == *task_id)
        .expect("No Task found");

    for (key, value) in config.env {
        std::env::set_var(key, value);
    }

    TaskRunner::new().run(task);
}
