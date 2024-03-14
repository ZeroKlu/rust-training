#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn valid(&self) -> bool {
        self.width() && self.height()
    }

    // Can use the same name as field
    fn width(&self) -> bool {
        self.width > 0.0
    }
    fn height(&self) -> bool {
        self.height > 0.0
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

    let prefix = if rect.valid() {""} else {"Invalid "};
    println!("{prefix}Area: {:.2}\n", rect.area());
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
