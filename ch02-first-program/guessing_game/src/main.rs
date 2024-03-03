use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Only use for debugging
    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");
    
        // 'mut' declares this as a mutable variable (omit for an immutable)
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // Result.expect() executes when the Result is Err (vs. Ok)
    
        // Shadowing allows reuse of the same variable name
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err (_) => continue
        };
        
        println!("You guessed: {guess}");
    
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
