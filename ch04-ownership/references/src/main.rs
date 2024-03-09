
fn main() {
    let s1 = String::from("hello"); // s1 comes into scope
    // Passing the reference pointer (&s) does not transfer ownership
    let len = calculate_length(&s1);
    println!("Length of '{s1}' is {len} characters."); // s1 is still valid

    // To allow changes in a borrowing function, we...
    let mut s2 = String::from("hello"); // ... make the variable mutable
    change(&mut s2); // and declare the reference as mutable
    let len = calculate_length(&s2);
    println!("Length of '{s2}' is {len} characters.");
} // s1 goes out of scope and is dropped, because it is still valid

// Accepting a reference (&String) does not take ownership
fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope but nothing is dropped here,
  //   because the function does not own s

// We also have to make the parameter mutable to allow changes
fn change(s: &mut String) {
    s.push_str(", world");
}