## Passing Multiple Parameters to a Method ##

Although the ```self``` argument is required, it does not have to be the only
argument to a method.

The ```self``` argument is implicit, but additional arguments are passed
just like to regular functions: ```object.method(arg(s))```

```rust
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    // Here, we're passing a float value for computation
    fn multiply(&self, multiple: f64) -> f64 {
        self.area() * multiple
    }

    // Here, we're passing another Rectangle to compare
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
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
    // Call to the multiply method with additional argument
    println!("Area of rect1 * {} = {}", multiple, rect1.multiply(multiple));

    // Calls to can_hold method with additional argument(s)
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

---
