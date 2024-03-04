### Loop Until the User Guesses the Number ###

```rust
use std::io;
use rand::Rng;
use std::cmp::Ordering; // Add the Ordering class for comparisons

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Move the logic into a loop
    loop {
        println!("Please input your guess.");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");
    
        let guess: u32 = guess.trim().parse()
            .expect("Please type a number!");
    
        println!("You guessed {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // End the loop once the user wins
                break;
            }
        }
    }
}
```
