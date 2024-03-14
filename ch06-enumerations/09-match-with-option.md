## Matching with ```Option<T>``` ##

We previously used ```match``` against a set of enum variants. We can do
the same thing using the variants of the built-in enum ```Option<T>```.

```rust
fn main() {
    let five = Some(5);
    let six = plus_one(five);  // Some(6)
    let none = plus_one(None); // None

    // ...
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // Matches must be exhaustive (You can't leave one out)
        None => None,
        Some(i) => Some(i + 1),
    }
}
```

---
