## Example Program - Printout: Add Debug Trait ##

An example program that computes the area of a rectangle

```rust
// Adding the 'Debug' trait to the Rectangle struct
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

fn main() {
    let mut width = -1.0;
    while width < 0.0 {
        width = get_float("Enter width: ")
    }
    let mut height = -1.0;
    while height < 0.0 {
        height = get_float("Enter height: ")
    }

    let rect = Rectangle {
        width,
        height,
    };

    let area = get_area(&rect);
    // Because we have the 'Debug' trait, we can now print the Rectangle
    println!("\n{:#?}", rect);
    println!("Area: {:.2}\n", area);
}

fn get_area(rectangle: &Rectangle) -> f64 {
    rectangle.width * rectangle.height
}

fn get_float(message: &str) -> f64 {
    println!("\n{}", message);
    let mut line = String::new();
    let _ = std::io::stdin().read_line(&mut line).unwrap();
    match line.trim().parse::<f64>() {
        Ok(num) => if num > -1.0 {num} else {-1.0},
        Err(_) => -1.0
    }
}
```

---

Note: The 'Debug' trait also allows the use of the ```dbg!()``` macro

```rust
// * SNIP *

    let rect = Rectangle {
        width,
        height,
    };

    dbg!(rect); // Prints out line number and value

// * SNIP *
```

---

Follow-up Reading:
[Rust Attributes](https://doc.rust-lang.org/reference/attributes.html)

---
