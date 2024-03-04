### Compare the Guess to the Seret Number ###

```rust
use std::io;
use rand::Rng;
use std::cmp::Ordering; // Add the Ordering class for comparisons

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");

    // Shadow the guess variable as an integer to allow comparison
    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");

    println!("You guessed {guess}");

    // Compare the guess to the secret number
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```
