### Generating a Seret Number ###

Edit Cargo.toml
```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"

[dependencies]
# Add the crate for generating random numbers
rand = "0.8.5"
```

```rust
use std::io;
use rand::Rng; // Adding in the namespace & trait

fn main() {
    println!("Guess the number!");

    // Generate the random secret number
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Only use this line for debugging
    println!("The secret number is {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");

    println!("You guessed {guess}");
}
```
