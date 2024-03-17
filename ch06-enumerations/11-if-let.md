## Concise Control Flow with ```if``` ```let``` ... ##

Sometimes a match block is unnecessarily verbose, like this:

```rust
fn main() {
    let config_max = Some(3u8);

    match config_max {
        // We're only handling the 'Some' variant
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (), // Do nothing with everything else is pointless boilerplate
    }
}
```

The code works, but we only have one condition where we specify an outcome.

---

We can express the same behavior more concisely, using ```if``` ```let``` ...

In this construct, we provide a pattern (acting like the first are of a
```match```) and an expression to evaluate against the pattern. After that,
this behaves just like a match where all other values are ignored.

We use the following syntax:

```if let <pattern> = <expression> { <code if matched> };```

```rust
fn main() {
    let config_max = Some(3u8);

    // Pattern:    Some(max)
    // Expression: config_max
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}
```

---

We can also supply an else block to handle alternate conditions.

```rust
fn main() {
    let config_max = Some(3u8);

    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    } else {
        println!("The maximum is not configured!");
    }
}
```

---
