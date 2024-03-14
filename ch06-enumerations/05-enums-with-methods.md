## Adding Methods to Enumerations ##

Like structs, we can define methods associated with enumerations.

```rust
#[derive(Debug)]
enum Message {
    Quit,                        // No data: Just a variant
    Move {x: i32, y: i32},       // Two named fields
    Write(String),               // One String value
    ChangeColor(i32, i32, i32),  // Three integer values
}

impl Message {
    // This adds a method to the enum
    fn call(&self) {
        // TODO: Behaviors for different variants
        println!("{:?}", self);
    }
}

fn main() {
    let w = Message::Write(String::from("hello"));
    // Calling the enum method
    w.call();
}
```

---
