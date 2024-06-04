## Refactoring and Handling Errors ##

We need to make sure that make sure we are writing code that is both
modular and capable of handling errors.

### Refactor Reading Arguments ###

Any function should only handle a single task. In our ```main()``` 
function, we are currently both extracting the arguments and reading the
file.

Both of these processes should be extracted into separate functions.

For the argument reading, we can implement a function like this:

```rust
fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}
```

Then we can modify ```main()``` to do the following:

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, file_path) = parse_config(&args);

    // --snip--
}
```

---

That's an improvement, but since we're returning a tuple and then 
immediately breaking the tuple into two variables, that's a sign that we
haven't yet hit the mark.

We can further improve the program by introducing a ```struct``` to
contain the argument values, like this:

```rust
struct Config {
    query: String,
    file_path: String,
}
```

And of course modify the functions to use the struct

```rust
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);
    
    let contents = fs::read_to_string(config.file_path)
}

fn parse_config(args: &[String]) -> Config {
    // We're cloning here to allow the Config object to own the attributes
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}
```

---

> ### The Trade-Offs of Using ```clone``` ###
>
> The choice to use ```clone``` here is a simple way to deal with the
> potential ownership issue when obtaining the values from ```args```.
> However, this is not optimized, because of the cost in memory use.

---

The ```parse_config()``` function's only purpose is to create an instance
of the ```Config``` struct, so it makes sense to replace the function with
a constructor.

At the same time, we can add some rudimentary error handling.

```rust
impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("Not enough arguments!");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
    
        Config { query, file_path }
    }
}
```

```rust
fn main() {
    // -- SNIP --
    let config = Config::new(&args);
    // -- SNIP --
}
```

---

Alternately, we could have the function return a ```Result``` instead
of panicking.

```rust
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
```

```rust
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // -- SNIP --
}
```

---

### Extracting Logic from ```main``` ###

Our ```main``` function is still performing the logic for reading the file.
We can extract that to a purpose-built function.

```rust
fn main() {
    // -- SNIP --
    
    run(config);
}

fn run(config: Config) {
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");
    println!("With text:\n{contents}");
}
```

---

We can further improve this by having the function return a
```Result```

```rust
use std::error::Error;

fn main() {
    // -- SNIP --

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Recall that `?` returns the error value instead of panicking
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{contents}");
    Ok(())
}
```

---

### Splitting Code into a Library Crate ###

It now makes sense to split out the functionality that is not part of
```main``` into a library crate.

In a new file (lib.rs), move the following:

* The ```run``` function
* The ```Config``` struct
* The ```Config::build``` function
* The relevant ```use``` statements
    * ```use std::fs;```
    * ```use std::error::Error;```

Be sure to add the ```pub``` keyword to the struct, its members, and both 
functions:

src/lib.rs

```rust
use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{contents}");
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
```

---

Then in main.rs:

* Bring ```Config``` into scope
    * ```use minigrep::Config;```
* Prefix library function calls with ```minigrep::```
    * ... ```minigrep::run(config)``` ...

src/main.rs

```rust
use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
```

---
