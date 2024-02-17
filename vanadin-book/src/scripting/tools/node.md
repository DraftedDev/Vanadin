# The `tools/node` Module

The `tools/node` module provides useful functions for working with [Node.js](https://nodejs.org/).

Import it with:

```js
import { run, ... } from "tools/node";
```

## Functions

---

### `run(file)`

Runs the given file in Node.js with extra development features. Basically just runs `node <file>`. Returns the exit or null if there was no exit code.

---

### `node(cmd)`

Runs the given command in Node.js. Basically just runs `node <cmd>`. Returns the exit or null if there was no exit code.

---

### `launch(file)`

Runs the given file in Node.js without extra development features. Basically just runs `node <file>`. Returns the exit or null if there was no exit code.

---
