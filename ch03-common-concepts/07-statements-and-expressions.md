## Statements and Expressions ##

A function body is made up of a series of statements optionally
ending in an expression (return value).

### Definitions ###

* **Statement:**<br>
  A statement is an instruction that performs an action but does not return a value.

* **Expression:**<br>
  An expression evaluates to a result (return) value.

Note: Rust is an expression-based language.

---

### Statements ###

Statements can be any instruction that does not return a value

Creating and assigning a variable is a statement
```rust
// The 'let' instruction does not return a value
let x = 6;

// So this would be an error
// let y = (let z = 6);
```

In many other languages, assignment returns the result of the assignment,
allowing chaining like the below example from Python, but Rust does not support this.

```python
# In Rust, you cannot do this
x = y = 123
```

A function definition itself is a statement
```rust
fn main() {
    // Code here
}
```

---

### Expressions ###

Expressions can be any instruction that evaluates to a value.

Note: The operand to the right of an assignment operator must be
an expression, but any expression (even a complex one like a
code block) is valid.

```rust
fn main() {
    // Although 'let' is a statement, '3 + 4' is an expression, since '3 + 4'
    //  evaluates to '7' before 'let' assigns the value to 'x'
    let x = 3 + 4;
}
```

A call to a macro is an expression.

```rust
fn main() {
    // println! is a macro (returning a TokenStream), so this is an expression
    println!("Hello world");
}
```

A code block (surrounded by curly braces) is an expression.

```rust
fn main() {
    // 'let' is still a statement
    let y = {
        // This code block is an expression
        let x = 3;
        x + 1 // No semicolon here, so the entire block evaluates to 4
    };
}
```

A function that returns a value is an expression

```rust
fn five() -> i32 {
    5
}
```

---
