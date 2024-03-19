## Anatomy of a Rust Program ##

**```main()```**

The *main()* function is always the first code that runs in
a Rust program.

```rust
// main() always runs first in a Rust program
fn main() {
    // Curly-braces encapsulate the block of code
}
```

---

**```println!```**

In Rust, ```println!()``` calls a macro (indicated by the
```!``` character).

The value ```"Hello, world!"``` is a literal string (indicated
by double-quotes ```"```").

Note: In Rust, the type of quotes used matters. Single quotes
(```'```) are used to indicate a character, not a string.

```rust
fn main() {
    // 'Hello, world!' is printed to the terminal
    println!("Hello, world!");
}
```

---



