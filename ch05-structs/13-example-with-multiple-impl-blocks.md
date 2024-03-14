## Multiple ``impl`` Blocks ##

You can add any number of ```impl``` blocks for a given struct.

Typically, this would be to logically group the associated functions and 
methods.

In this example, I have grouped the constructors and methods separately.

```rust
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle { // Methods
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn multiply(&self, multiple: f64) -> f64 {
        self.area() * multiple
    }
}

impl Rectangle { // Constructors
    fn new(width: f64, height: f64) -> Self {
        Self {
            width,
            height,
        }
    }

    fn square(side: f64) -> Self {
        Self {
            width: side,
            height: side,
        }
    }
}

fn main() {
    let rect1 = Rectangle{
        width: 30.0,
        height: 50.0,
    };
    let rect2 = Rectangle{
        width: 10.0,
        height: 40.0,
    };
    let rect3 = Rectangle{
        width: 60.0,
        height: 45.0,
    };

    println!("Area of rect1 = {}", rect1.area());

    let multiple = 2.0;
    println!("Area of rect1 * {} = {}", multiple, rect1.multiply(multiple));

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let rect4 = Rectangle::new(5.0, 7.0);
    println!("rect4 = {:#?}", rect4);
    
    let square = Rectangle::square(6.0);
    println!("square = {:#?}", square);
}
```

---
