# `sys` Module

The `sys` module provides functions to retrieve system information.

To import the `sys` module, use the `import` keyword:

```js
import { cpuCount, ... } from 'sys';
```

## Functions

**NOTE:** Most or all of these functions are quite expensive, since they request system information from the OS and are not cached.

---

### `cpuCount()`

Returns the number of CPUs in the system or `null` if it cannot be determined.

---

### `cpuBrands()`

Returns an array of CPU brands in the system.

---

### `cpuNames()`

Returns an array of CPU names in the system.

---

### `cpuFrequencies()`

Returns an array of CPU frequencies in the system.

---

### `cpuUsages()`

Returns an array of CPU usage in the system.

---

### `availableMemory()`

Returns the available memory in the system.

Generally, free memory refers to unallocated memory whereas available memory refers to memory that is available for (re)use.

---

### `freeMemory()`

Returns the free memory in the system.

Free memory refers to unallocated memory whereas available memory refers to memory that is available for (re)use.

Windows doesn't report free memory so this method returns the same value as `availableMemory()`.

---


### `totalMemory()`

Returns the total memory in the system.

---

### `usedMemory()`

Returns the used memory in the system.
