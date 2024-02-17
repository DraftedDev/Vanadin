# The `tools/cargo` Module

The `tools/cargo` module provides useful functions for working with [Rust & Cargo](https://rust-lang.org).

Import it with:

```js
import { run, ... } from "tools/cargo";
```

## Functions

---

### `cargo(cmd)`

Runs the given command and returns the exit code if given, otherwise `null`.

---

### `run()`

Executes `cargo run` and returns the exit code if given, otherwise `null`.

---

### `build()`

Executes `cargo build` and returns the exit code if given, otherwise `null`.

---

### `test(test)`

Executes `cargo test --test <test>` or `cargo test` if no test is given and returns the exit code if there is one, otherwise `null`.

---

### `clean()`

Executes `cargo clean` and returns the exit code if given, otherwise `null`.

---

### `release()`

Builds the crate in release mode with `--config strip=true --future-incompat-report` and returns the exit code if given, otherwise `null`.
