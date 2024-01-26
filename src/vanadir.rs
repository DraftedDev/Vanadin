use std::fs;
use std::path::PathBuf;

use toml::{Table, Value};

use vanadin_tasks::task::Task;

pub const VANADIR: &str = ".vanadin";
pub const CONFIG_FILE: &str = "Vanadin.toml";
pub const ENV_SECTION: &str = "env";
pub const TASK_SECTION: &str = "task";

pub struct VanadinConfig {
    pub env: Vec<(String, String)>,
    pub tasks: Vec<Task>,
}

impl VanadinConfig {
    pub fn fetch() -> Self {
        let config: Value = toml::from_str(
            fs::read_to_string(get_vanadir().join(CONFIG_FILE))
                .expect("Failed to read Vanadin.toml")
                .as_str(),
        )
        .expect("Failed to parse Vanadin.toml");

        let empty_table = Value::from(Table::new());

        let env: &Table = config
            .get(ENV_SECTION)
            .unwrap_or(&empty_table)
            .as_table()
            .expect("Failed to parse env section");

        let task: &Table = config
            .get(TASK_SECTION)
            .unwrap_or(&empty_table)
            .as_table()
            .expect("Failed to parse task section");

        Self {
            env: env
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        v.as_str().expect("Failed to parse env value").to_string(),
                    )
                })
                .collect(),

            tasks: task
                .iter()
                .map(|(k, _)| construct_task(&config, k.as_str()))
                .collect(),
        }
    }
}

pub fn get_vanadir() -> PathBuf {
    std::env::current_dir()
        .expect("Current directory not found")
        .join(VANADIR)
}

pub fn construct_task(root: &Value, name: &str) -> Task {
    let task = root.get(TASK_SECTION).expect("Failed to find task section");

    Task {
        id: task
            .get("name")
            .unwrap_or(&Value::from(name))
            .as_str()
            .expect("Failed to parse task id")
            .to_string(),

        about: task
            .get("about")
            .unwrap_or(&Value::from(""))
            .as_str()
            .expect("Failed to parse task about")
            .to_string(),

        src: fs::read_to_string(
            task.get("src")
                .unwrap_or(&Value::from(
                    get_vanadir()
                        .join("tasks")
                        .join(format!("{}.js", name))
                        .to_str()
                        .unwrap(),
                ))
                .as_str()
                .expect("Failed to parse task src"),
        )
        .expect("Failed to read task")
        .to_string(),

        pre_run: task
            .get("pre")
            .unwrap_or(&Value::from(Vec::<String>::new()))
            .as_array()
            .expect("Failed to parse pre")
            .iter()
            .map(|v| construct_task(root, v.as_str().expect("Failed to parse pre task name")))
            .collect(),

        post_run: task
            .get("post")
            .unwrap_or(&Value::from(Vec::<String>::new()))
            .as_array()
            .expect("Failed to parse post")
            .iter()
            .map(|v| construct_task(root, v.as_str().expect("Failed to parse pre task name")))
            .collect(),
    }
}
