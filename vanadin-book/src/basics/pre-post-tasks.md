# Pre- and Post-Tasks

You can define pre- and post-tasks to execute before and after the main task.

These tasks can be defined in the `[task]` section like this:

```toml
[task.prepare]
about = "Prepare your meal"

[task.eat]
about = "Eat your meal"
pre = ["prepare"]
post = ["cleanup"]

[task.cleanup]
about = "Clean up after your meal"
```

When running the `eat` task, the `prepare` and `cleanup` tasks will be executed before and afterward.

**NOTE 1:** The `pre` and `post` tasks will be executed in the order they are defined.

**NOTE 2:** Make sure that pre- and post-tasks do not depend on each other, so that infinite loops may occur.
