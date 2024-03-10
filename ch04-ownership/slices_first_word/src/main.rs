fn main() {
    let s = String::from("hello world");
    println!("{}", first_word_length(&s));
}

fn first_word_length(s: &String) -> usize {
    let bytes = s.as_bytes();

    // iter() returns each element in a collection
    // enumerate() wraps iter() to return tuples of (index, &element)
    for (i, &item) in bytes.iter().enumerate() {
        // Return the position of the first space in the string
        if item == b' ' {
            return i;
        }
    }
    // If no spaces were found, return the length of the string
    s.len()
}
