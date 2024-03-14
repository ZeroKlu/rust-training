## Associated Functions ##

You can add functions to a struct that do not take the ```self``` argument and
are therefore not methods.

These are like *static* methods in other languages and are called from the
struct, not from an instance.

One common example of this is a constructor.

Note: In Rust, the convention is to name a constructor ```new```, but unlike
many other languages, in Rust, 'new' is not a reserved word.

```rust
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    // * SNIP *

    // Constructor
    fn new(width: f64, height: f64) -> Self {
        Self {
            width,
            height,
        }
    }

    // Constructor (not called 'new')
    fn square(side: f64) -> Self {
        Self {
            width: side,
            height: side,
        }
    }
}

fn main() {
    // * SNIP *

    // Call associated function (constructor)
    let rect4 = Rectangle::new(5.0, 7.0);
    println!("rect4 = {:#?}", rect4);
    
    // Call associated function (square constructor)
    let square = Rectangle::square(6.0);
    println!("square = {:#?}", square);
}
```

---
