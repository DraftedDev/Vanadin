# `process` Module

The `process` module provides useful functions for spawning processes, running commands and more.

To import the `process` module, use the `import` keyword:

```js
import { cmd } from 'process';
```

## Functions

---

### `cmd(command, args)`

Runs the given command with the given arguments and returns the exit code. Returns `null` if an error occurs.

---

### `exit(code)`

Exits the process with the given exit code.

---

### `id()`

Returns the current process ID.
