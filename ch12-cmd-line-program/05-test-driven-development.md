## Developing the Library’s Functionality with Test-Driven Development ##

When adding logic to a program, it can be useful to build it using 
test-driven development (TDD), which follows this pattern:

1. Write a test that fails and run it to make sure it fails for the reason 
   you expect.
2. Write or modify just enough code to make the new test pass.
3. Refactor the code you just added or changed and make sure the tests 
   continue to pass.
4. Repeat from step 1...

---

### Writing a Failing Test ###

We can start by creating a test function that we know will fail.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
```

This function will fail to even compile, since we don't yet have a
```search()``` function, so ```cargo test``` returns the following:

```
error[E0425]: cannot find function `search` in this scope
  --> src\lib.rs:40:53
   |
40 |         assert_eq!(vec!["safe, fast, productive."], search(query, contents));
   |                                                     ^^^^^^ not found in this scope
```

---

So, we add just enough code to get to the next step...

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}
```

After creating the function (with minimal code), we can compile, but
```cargo test``` still fails:

```
...

running 1 test
test tests::one_result ... FAILED

failures:

---- tests::one_result stdout ----
thread 'tests::one_result' panicked at src\lib.rs:44:9:
assertion `left == right` failed
  left: ["safe, fast, productive."]
 right: []

 ...
```

---

### Writing Code to Pass the Test ###

Now we update the ```search()``` function to make sure it returns the
expected results:

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
```

```
...

Running unittests src\lib.rs (target\debug\deps\minigrep-518fa49251c7d54b.exe)

running 1 test
test tests::one_result ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

...
```

---

### Using the ```search``` Function in the ```run``` Function ###

Now that the ```search``` function works, we can add a call to it in the
```run``` function:

```rust
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}
```

And now, we can execute the program:

```
cargo run -- body .\data\poem.txt
```

```
Searching for 'body'
In file .\data\poem.txt

I'm nobody! Who are you?
Are you nobody, too?
How dreary to be somebody!
```

---
