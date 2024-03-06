fn main() {
    // Calls to separate functions
    hello_world();
    hello_name("Programmer");
    labeled_measure(42, 'g');

    // Calls to separate functions with return values
    let n = 2;
    let x = square_explicit(n);
    println!("{n}² = {x}");
    let y = square(x);
    println!("{x}² = {y}");
    let p = pi();
    println!("pi = {p}");
}

// Separate function declaration
fn hello_world() {
    println!("Hello, World!");
}

// Function that accepts an argument
fn hello_name(name: &str) {
    println!("Hello, {name}!");
}

// You can pass multiple arguments
fn labeled_measure(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// You can explicitly use the return keyword to return a value
fn square_explicit(n: i32) -> i32 {
    return n * n;
}

// Or you can omit the semicolon from the final expression for the same result
fn square(n: i32) -> i32 {
    n * n
}

// Here, we have no instructions at all, just a literal being returned
fn pi() -> f64 {
    3.14159
}