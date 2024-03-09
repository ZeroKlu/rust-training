fn main() {
    let s = String::from("hello");
    // Passing the reference pointer (&s) does not transfer ownership
    let len = calculate_length(&s);
    println!("Length of '{s}' is {len} characters."); // s is still valid
}

fn calculate_length(s: &String) -> usize {
    s.len()
}