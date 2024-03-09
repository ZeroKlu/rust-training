fn main() {
    let s1 = String::from("hello");
    // We can work around the problem of passing ownership in some cases
    //   by returning both the computed result and the original value
    let (s2, len) = calculate_length(s1);
    println!("Length of '{s2}' is {len} characters.");
}

// Rust allows returning multiple values (in a tuple)
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}