# Built-in Features

Vanadin provides a set of useful features by default.

Following functions may be called without extra module imports:
```javascript
print("Some String"); // prints "Some String"
println("Another String"); // prints "Another String" with a newline at the end
assert(condition); // tests if condition is true and throws an error otherwise
throw("Error Message"); // throws an error with the given message
```

There are also pre-defined constants:
```javascript
println(ID); // Prints the ID/name of the current task
println(ABOUT); // Prints the description of the current task
```
