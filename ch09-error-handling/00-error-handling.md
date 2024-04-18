## Error Handling ##

In Rust, instead of handling exceptions (try/catch), the
developer must address potential error conditions before
compiling.

There are two types of error conditions:

* Recoverable: Report problem and continue executing
* Unrecoverable: Halt execution

For recoverable (non-fatal) errors, Rust provides the
```Result<T, E>``` type.

Fon unrecoverable (fatal) errors, Rust provides the
```panic!``` macro.

It is up to the programmer to determine the appropriate usage.

---
