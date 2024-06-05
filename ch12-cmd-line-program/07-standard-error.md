## Writing Error Messages to Standard Error Instead of Standard Output ##

As with most languages, in Rust the standard output (```stdout```) and 
standard error (```stderr```) are separate buffers.

Thus far, we have been displaying all of our messages with ```println!```, 
which prints to ```stdout```.

It can be useful to display error messages using ```stderr``` instead, as
the two output buffers can be redirected to write to different locations.

---

For example, if we redirect the output of the program to a file, like this:

```
cargo run -- to .\data\poem.txt > .\data\results.txt
```

Our results will be written to the file data/results.txt

In data/results.txt

```
Are you nobody, too?
How dreary to be somebody!

```

---

However, if we forget an argument and produce an error, like this:

```
cargo run -- to > .\data\results.txt
```

The error is not shown in the terminal, but is written to the file
(just like the normal output).

In data/results.txt

```
Problem parsing arguments: not enough arguments
```

In the terminal, we only see the non-specific error

```
error: process didn't exit successfully: `target\debug\minigrep.exe to` (exit code: 1)
```

---

### Printing Errors to Standard Error ###

The Rust standard library provides a separate print macro that points to
```stderr``` named ```eprintln!()```

So we'll modify the ```main``` function as follows

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        // Use `eprintln!` for printing errors
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        // Use `eprintln!` for printing errors
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
```

---

Now, when we redirect the output to the file, when it completes
successfully, nothing changes. The output is written to file.

However, when we cause an error:

```
cargo run -- to > .\data\results.txt
```

The error appears in the terminal, not the file (which is empty).

```
Problem parsing arguments: not enough arguments
error: process didn't exit successfully: `target\debug\minigrep.exe to` (exit code: 1)
```

---
