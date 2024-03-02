use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // 'mut' declares this as a mutable variable (omit for an immutable)
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); // Result.expect() executes when the Result is Err (vs. Ok)

    println!("You guessed: {guess}");
}
