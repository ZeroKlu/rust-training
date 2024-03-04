### Processing a Guess ###

```rust
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // By default, Rust variables are immutable
    // The 'mut' keyword declares the variable as mutable
    let mut guess = String::new();

    // read_line() returns a Result (enum: Ok | Err)
    //             .expect() executes if Result is Err
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");

    println!("You guessed {guess}");
}
```
