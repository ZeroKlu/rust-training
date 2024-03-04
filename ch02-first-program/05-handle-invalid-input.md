### Handle Case Where User Enters Invalid Data ###

```rust
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");
    
        // Use match to evaluate the Result value from parse() to determine if the data is valid
        let guess: u32 = match guess.trim().parse() {
            // On Ok, we return the integer value
            Ok(num) => num,
            // On Err, we ignore the value and continue the loop
            Err (_) => continue,
        };
    
        println!("You guessed {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```
