## Working with Environment Variables ##

We can include environment variables in our command-line calls to Cargo.

Here, we're adding the ability to specify a case-insensitive search using
the ```IGNORE_CASE``` environment variable.

---

Sticking with the TDD process, we'll start by adding a test for a 
case-insensitive search.

In src/lib.rs

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Rename the existing test to `case_sensitive`
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    // Add a test for `case_insensitive` search
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
```

---

Of course, this will currently fail to compile, since we haven't added a
```search_case_insensitive``` function to the library yet.

We'll implement that function like this:

In src/lib.rs

```rust
pub fn search_case_insensitive<'a>(query: &str,contents: &'a str,) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
```

---

Now the test passes, but we still have not implemented the ability to use
this function in the program.

This involves a few steps.

---

First, we add a member ```ignore_case``` to our ```Config``` struct

```rust
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}
```

---

Next, we modify the ```run``` function to use the new member

```rust
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    // Check the `ignore_case` value and call the proper function
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}
```

---

Then we modify the ```Config::build``` function to check the environment
variable.

```rust
use std::env;
// -- SNIP --

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        // Check `IGNORE_CASE` environment variable
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
```

---

Now, if we run ```cargo run -- to .\data\poem.txt```, we still see our
original results:

```
Are you nobody, too?
How dreary to be somebody!
```

---

However, if we specify the ```IGNORE_CASE``` variable as true (1), like
this:

```
$Env:IGNORE_CASE=1; cargo run -- to .\data\poem.txt
```

... we also capture the instances of 'To' that contain capitals

```
Are you nobody, too?
How dreary to be somebody!        
To tell your name the livelong day
To an admiring bog!
```

---

The ```$Env:IGNORE_CASE=1;``` part of the command has set the variable
for the duration of the session in PowerShell, so we need to delete
that setting to go back to case-sensitive search:

```
Remove-Item Env:IGNORE_CASE
```

---

