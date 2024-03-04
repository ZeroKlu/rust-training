fn main() {

    // *** Mutability *** //

    let x = 1;
    println!("The value of x is: {x}");
    
    // This will cause an error, because x is immutable
    // x = 2;
    // println!("The value of x is: {x}");

    // With the 'mut' keyword, we can explicitly make a variable mutable
    let mut y = 3;
    println!("The value of y is: {y}");

    y = 4;
    println!("The value of y is: {y}");

    // *** Constants *** //

    const SECONDS_PER_HOUR: u32 = 60 * 60;
    let hours = 3;
    let seconds = hours * SECONDS_PER_HOUR;
    println!("There are {seconds} seconds in 3 hours");

    // *** Shadowing *** //

    let num = 5;

    // Note: With let, we can replace the immutable variable
    // Techincally, this is also a shadow, but since it's in the same
    //   scope, it is the only one visible to the compiler
    let num = num + 1;
    
    {
        // Within inner scope, this num shadows the existing num
        let num = num * 2;
        println!("Inner num = {num}"); // 12
    }
    
    // After exiting the scope, the original variable is unshdowed
    println!("Outer num = {num}"); // 6

    // We can shadow with a different data type for example,
    //   provided we only need the integer value later in the program
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces = {spaces}"); // 3, not '   '
}
