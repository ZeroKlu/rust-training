## Example Program - Adding a Method ##

An example program that computes the area of a rectangle

We can add a method to a struct as seen below.

Notes:

The first argument to the method is either:
* ```&self``` if not modifying the struct<br>or
* ```&mut self``` to borrow as mutable

```rust
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

// Here we're adding a method to the struct
impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
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

    println!("\n{:#?}", rect);
    // Call the method
    println!("Area: {:.2}\n", rect.area());
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
