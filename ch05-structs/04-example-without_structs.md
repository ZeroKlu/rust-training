## Example Program - No Structs ##

An example program that computes the area of a rectangle

```rust
fn main() {
    let mut width = -1.0;
    while width < 0.0 {
        width = get_float("Enter width: ")
    }
    let mut height = -1.0;
    while height < 0.0 {
        height = get_float("Enter height: ")
    }

    let area = get_area(width, height);
    println!("\nArea of a {} by {} rectangle = {}", width, height, area);
}

// Because these are two separate parameters, it's unclear that they
//   relate to a single rectangle
fn get_area(width: f64, height: f64) -> f64 {
    width * height
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