use std::fs;
use std::path::Path;

pub fn init() {
    let root = Path::new(".vanadin");

    fs::create_dir(root).expect("Failed to create .vanadin directory");

    fs::write(
        root.join("Vanadin.toml"),
        r#"# The config file for your project

# Specify global environment variables
[env]
GREET = "Joe"

# Specify a task using a [task.<name>] block
[task.build]
name = "build"
about = "Builds the project"
src = "./tasks/build.js"
"#,
    )
    .expect("Failed to create .js file");

    fs::create_dir(root.join("tasks")).expect("Failed to create .vanadin/tasks directory");

    fs::write(
        root.join("tasks").join("build.js"),
        r#"// The script for the build task
import { get } from 'env';

let name = get("GREET");
println(`Hello ${name}!`);
"#,
    )
    .expect("Failed to create .vanadin/tasks/build.js file");
}
