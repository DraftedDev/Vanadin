# Your First Project

## Initializing Vanadin

To get started, you first need to create a basic `.vanadin` folder.

To do so, run: 
```shell
$ vanadin init
```

## Anatomy of your `.vanadin`

The `.vanadin` folder is where you define tasks, set environment variables and configure your project.

It should look this:

```
My Project
├── .vanadin
│   ├── Vanadin.toml
│   └── tasks
│       └── build.js
└── ...
```

## The `Vanadin.toml` file

The `Vanadin.toml` file defines the tasks, environment variables and other stuff for Vanadin. 

Tasks are defined like this:
```toml
[task.task-name]
name = "task-name" # The name of the task. Defaults to the name of the [task.<name>] section
about = "About the task aka the its description" # Defaults to an empty string.
src = "./tasks/my-task.js" # This defaults to './tasks/<task-name>.js' if not specified
```

## The `tasks` folder

This is the default location of all tasks.

You should define tasks in this folder, but if you want to define them elsewhere, make sure to set the `src` field in your `[task]` section.

## Environment variables

Environment variables are set in the `[env]` section of your `Vanadin.toml` file:

```toml
[env]
MY_VARIABLE = "my-value"
SECRET_NUMBER = "123"
```

These variables are set before the execution of a task, so you can use them in the task.

## Running tasks

To run a task, execute:
```shell
$ vanadin x -t your-task-name
```

When creating your `.vanadin` with `vanadin init`, you can run the generated `build` task via `vanadin x -t build`.
