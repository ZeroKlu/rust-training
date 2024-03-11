fn main() {
    let s = String::from("hello world");

    let word_len = first_word_length(&s);
    println!("First word ends at: {word_len}");

    let word = first_word(&s);

    // This would cause an error, since s is already borrowed
    // s.clear();
    println!("First word is '{word}'");
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

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        // Return the slice containing the first word in the string
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
    // If no spaces were found, return a slice containing the whole string
}
