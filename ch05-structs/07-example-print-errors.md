## Example Program - Printout: Errors ##

An example program that computes the area of a rectangle

---

Attempting to print the rectangle results in an error

```rust
struct Rectangle {
    // * SNIP *
}

fn main() {
    let rectangle = new Rectangle{
        // * SNIP *
    }

    // Error: trait `std::fmt::Display` not implemented for `Rectangle`
    // Hint:  ... use `{:?}` (or {:#?} for pretty-print) instead
    println!("{}", rectangle);
}

// * SNIP *
```

---

Modifying based on the previous hint still doesn't work

```rust
struct Rectangle {
    // * SNIP *
}

fn main() {
    let rectangle = new Rectangle{
        // * SNIP *
    }

    // Error: trait `Debug` is not implemented for `Rectangle`
    // Hint:  consider annotating `Rectangle` with `#[derive(Debug)]`
    println!("{:?}", rectangle);
}

// * SNIP *
```

---
