## Reading a File ##

We'll create a test file (poem.txt) in a folder (data) with the following 
content:

```
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```

---

To read the file, we'll modify our code with the following:

```rust
use std::env;
use std::fs;

fn main() {
    // --snip--
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
```

---

Test by running the following command:

```
cargo run -- nobody .\data\poem.txt
```

---
