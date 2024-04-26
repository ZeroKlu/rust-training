fn main() {
    // Variables live on the stack
    let n = 5; // Stack: main[n|5]
    let y = plus_one(n); // Stack: main[n|5],[y|6]
    println!("The value of y is: {y}");

    let a = 5; // Stack: main[a|5] (n & y are out of scope - after last use)
    let mut b = a; // Stack: main[a|5],[b|5]
    b += 1; // Stack: main[a|5],[b|6]
}

fn plus_one(x: i32) -> i32 {
    // Stack: main[n|5], plus_one[x|5]
    x + 1
}
