fn main() {
    // Declare a variable
    let x = 5;
    println!("x = {x}");

    // This will fail because x is immutable
    // x = 6;
    // println!("x = {x}");

    // Declare a variable
    let mut y = 5;
    println!("y = {y}");

    // Mutate a variable
    y = 6;
    println!("y = {y}");

    // This will not work , because we cannot mutate the data type
    // y = "abc";

    // Declare constants
    const MINUTE_IN_SECONDS: u32 = 60;
    const HOUR_IN_MINUTES: u32 = 60;
    const HOUR_IN_SECONDS: u32 = HOUR_IN_MINUTES * MINUTE_IN_SECONDS;

    // Use a constant
    let three_hours = HOUR_IN_SECONDS * 3;
    println!("Three hours = {three_hours} seconds.");
}
