# The `tools/python` Module

The `tools/python` module provides useful functions for working with [Python](https://python.org/).

Import it with:

```js
import { run } from "tools/python";
```

## Functions

---

### `run(file)`

Runs the given file or `__main__.py` if `null` is given. Executes the file with extra development features and returns the exit code or `null` if none was provided.

---

### `py(cmd)`

Runs the given command using the `python` command. Returns the exit code or `null` if none was provided.

---

### `launch(file)`

Runs the given file or `__main__.py` if `null` is given. Executes the file with extra optimization and returns
the exit code or `null` if none was provided.

---
