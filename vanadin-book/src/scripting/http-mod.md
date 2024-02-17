# `http` Module

The `http` module provides functions to work with the HyperText Transfer Protocol (HTTP).

To import the `http` module, use the `import` keyword:

```js
import { serve, ... } from 'http';
```

## Functions

---

### `serve({}, func)`

Starts an HTTP server with the given function to handle requests and an optional configuration object.

The default config looks like this:
```js
{
    address: 'http://127.0.0.1:8080',
    log: true
}
```

The `func` function only takes one argument: the `req` object.

This object looks like this:
```js
{
    url: 'http://127.0.0.1:8080/',
    method: 'GET',
    headers: [] // list of headers
}
```

You can also close this server by going to `<server-address>/close-vanadin-server` where `<server-address>` is the address of the server.
