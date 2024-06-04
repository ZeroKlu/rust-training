## Accepting Command-Line Arguments ##

We want the program to accept two command-line arguments:

* A string to search for
* A file to search in

Like this:

```
cargo run -- search-string example-file.txt
```

---

### Reading Argument Values ###

To read command-line arguments, we can implement the following:

```rust
// Bring the `environment` namespace into scope
use std::env;

fn main() {
    // Collect the arguments as a String Vector
    let args: Vec<String> = env::args().collect();
    // Note: args[0] is the program itself, so our passed arguments
    //       start at args[1]

    // Using the debug macro to print the arguments
    dbg!(args);
}
```

---

### Storing Argument Values in Variables ###

We can capture the arguments as follows:

```rust
    let query = &args[1];
    let file_path = &args[2];

    // Test that we obtained the expected values
    println!("Searching for {}", query);
    println!("In file {}", file_path);
```

As noted above, we ignore ```args[0]``` because it contains the executable
itself.

---

