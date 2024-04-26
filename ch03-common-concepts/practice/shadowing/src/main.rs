fn main() {
    let x = 5;
    println!("Initial x = {x}");

    // Creates new variable shadowing (not mutating) `x`
    let x = x + 1;
    println!("Shadow x = {x}");

    {
        // Inner scope shadows for duration of block
        let x = x * 2;
        println!("Inner scope x = {x}");
    }
    
    println!("x = {x}");

    // We can shadow with a different data type
    let s = "hello";
    println!("s = {s}");
    
    let s = s.len();
    println!("Shadow s = {s}");
}