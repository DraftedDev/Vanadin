# Coding in JavaScript

You may have noticed, that all tasks end with `.js`. This is because all tasks are written in plain old, easy & fast JavaScript.

Vanadin uses the [QuickJS](https://bellard.org/quickjs/) Engine under the hood to execute fast and tiny JavaScript code with a tiny runtime and small footprint.

The engine supports most of the [ES2023](https://tc39.es/ecma262/2023/) specification. Due to most tasks not requiring advanced features, Vanadin only supports most basic JavaScript with BaseObject support.

It should also be kept in mind, that scripts are executed in non-global scope and are not strict by default.
