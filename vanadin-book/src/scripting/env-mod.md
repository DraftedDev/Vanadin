# `env` Module

The `env` module provides access to environment variables and basic process variables.

To import the `env` module, use the `import` keyword:
```js
import { get, set, ... } from 'env';
```

## Functions

---

### `get(key)`

Returns the value of the environment variable with the name `key`.

---

### `set(key, value)`

Sets the value of the environment variable with the name `key` to `value`.

---

### `remove(key)`

Removes the environment variable with the name `key`.

---

### `dir()`

Returns the current working directory.

---

### `os()`

Returns the operating system name.

---

### `arch()`

Returns the architecture.

---

### `family()`

Returns the operating system family.
