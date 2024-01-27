# `fs` Module

The `fs` module provides functions for interacting with the file system.

To import the `fs` module, use the `import` keyword:

```js
import { readFile, ... } from 'fs';
```

## Functions

---

### `readFile(file)`

Returns the contents of the given file as a string or `null` if an error occurs.

---

### `writeFile(file, content)`

Writes the given content to the given file.

---

### `readFileBytes(file)`

Returns the contents of the given file as a byte array or `null` if an error occurs.

---

### `writeFileBytes(file, content)`

Writes the given content to the given file as a byte array.

---

### `createFile(file)`

Creates a new file with the given name.

---

### `removeFile(file)`

Deletes the given file.

---

### `createDir(dir)`

Creates a new directory with the given name.

---

### `removeDir(dir)`

Deletes the given directory.

---

### `exists(file)`

Checks if the given file exists.
