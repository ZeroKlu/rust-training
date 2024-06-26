## Tuple and Memberless Structs ##

### Tuple Structs ###

Rust supports structs without field names. These resemble tuples but retain
the benefit of struct naming.

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    // You can destructure a tuple struct just like a tuple
    let Color(r, g, b) = black;

    let origin = Point(0, 0, 0);
    // You can use tuple-style dot syntax to access elements
    let x = origin.0; // [L1]
}
```

<img src="../additional-files/images/diagram0501d.png"
     style="width:120px;" alt="Diagram 5.1d"
     title="Diagram 5.1d">
<br><sup><sup>[Diagram from Brown University](https://rust-book.cs.brown.edu)</sup></sup>

An advantage to using tuple structs rather than just tuples is that structs 
define individual data types, so a function that takes a Color as a parameter
cannot accidentally take a Point, even though both contain the same tuple-like
values.

---

### Unit-Like Structs (Without Fields) ###

A struct with no members behaves similarly to the unit type ```()``` and is 
used when there is a need to implement a trait on a type without storing any
data in it.

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual; // [L1]
}
```

<img src="../additional-files/images/diagram0501e.png"
     style="width:160px;" alt="Diagram 5.1e"
     title="Diagram 5.1e">
<br><sup><sup>[Diagram from Brown University](https://rust-book.cs.brown.edu)</sup></sup>

---
