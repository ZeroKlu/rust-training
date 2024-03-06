## Functions ##

### Naming Convention ###

* Function names should be in snake_case

---

### Declaration ###

Functions are declared using the ```fn``` keyword

```rust
fn main() {
    // Call to separate function
    hello_world();
}

// Separate function declaration
fn hello_world() {
    println!("Hello World!");
}
```

---

### Function Parameters ###

Functions can accept any number of arguments.

Arguments must include data type annotations.

```rust
// Function with a parameter
fn hello_name(name: &str) {
    println!("Hello {name}!");
}

// Function with multiple parameters
fn labeled_measure(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
```

---

### Returning Values ###

To return a value from a function:

1. Define the return type preceded by the arrow operator ```->```
2. Return the value by either:
   1. Using the ```return``` keyword followed by a value<br>or
   2. Omitting the semicolon from the last expression

```rust
// Do this...
fn square(n: i32) -> i32 {
    return n * n;
}

// or this...
fn square(n: i32) -> i32 {
    // If this line was followed by a semicolon, it would be a statement,
    //  and this would fail to compile, as there would be no return.
    n * n
}

// or even this...
fn pi() -> f64 {
    3.14159
}
```

