fn main() {
    // For a signed, 8-bit integer, the max value is 127 (0111 1111)
    let mut x: i8 = 127;
    println!("{x} = {x:b}");

    // What happens when we try to add 1 here depends on the compile mode
    //      Debug:      Runtime panic -> 'attempt to add with overflow'
    //      Release:    Overflows to -128 (1000 0000)
    x += 1;
    println!("{x} = {x:b}");

    // The same thing can occur at the minimum value -128 (1000 0000)
    x = -128;
    println!("{x} = {x:b}");

    // What happens when we try to subtract 1 here depends on the compile mode
    //      Debug:      Runtime panic -> 'attempt to subtract with overflow'
    //      Release:    Overflows to 127 (0111 1111)
    x -= 1;
    println!("{x} = {x:b}");
}
