## Slices ##

A slice is a contiguous sequence of elements from a collection rather than
the whole collection itself.

---

To start with, we'll examine a program that attempts to get the length of the
first word in a string:

```rust
fn main() {
    let s = String::from("hello world");

    let word_len = first_word_length(&s);
    println!("The first word of 's' is {word_len} characters long.");
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
```

We have a problem though. The ```usize``` returned by the function is not a
reference, so it's uncoupled from its context (the ```&String```).

The following modification would allow the code to compile and run, but would
result in invalid output.

```rust
fn main() {
    let s = String::from("hello world");

    let word_len = first_word_length(&s);

    s.clear();

    // This is incorrect, since word_len is still 5, but s is 0 characters
    println!("The first word of 's' is {word_len} characters long.");
}

fn first_word_length(s: &String) -> usize {
    // * SNIP *
}
```

If we tried to use ```word_len``` to obtain the word from ```s```, it would 
fail. The data is out of sync.

Things could get even worse if we tried to use the index to get the second
word, like this:

```rust
fn next_word_length(s: &String, start: usize) -> usize {
    // * SNIP *
}
```

---

