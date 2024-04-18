## Recoverable Errors with ```Result``` ##

When an error is not serious enough to crash the program,
Rust provides the ```Result``` enum, which is defined as
follows:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

In the above, ```T``` represents the data type to be returned
onn success (in the ```Ok``` variant), and ```E``` represents 
the error type that will be returned on error (in the 
```Err``` variant).

---

A common function that returns a ```Result``` is opening a
file. This returns a ```Result``` because the operation could
fail if (for example) the file is not found.

Consider this naive code

```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");
}
```

In this case, ```greeting_file_result``` will contain a
```Result``` object.

On ```Ok```, it will contain a file handle (
```std::fs::File```).

On ```Err```, it will contain  an io error (
```std::io::Error```)

---

To properly handle the example above, we should modify the
code to perform different tasks depending on the result.

```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
```

For the ```Ok``` branch, ```greeting_file``` will contain the 
inner file pointer from the ```Result```

For the ```Err``` branch, we're passing the error along to the
```panic!``` macro, where we would receive this error:

```
thread 'main' panicked at 'Problem opening the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:8:23
```

---

### Matching on Different Errors ###

In the above example, we're not paying any attention to the
error itself. We're just panicking when any error occurs.

Using the ```io::ErrorKind``` enum, we can interrogate the
error and provide a more meaningful panic message:

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // If the file wasn't found, we try to create it
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}
```

---

> Note: Closures (covered in a later chapter) can provide a
> more succinct way to express the same logic we used above
> in the cluttered nested matches.
>
> ```rust
> use std::fs::File;
> use std::io::ErrorKind;
> 
> fn main() {
>     let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
>         if error.kind() == ErrorKind::NotFound {
>             File::create("hello.txt").unwrap_or_else(|error| {
>                 panic!("Problem creating the file: {:?}", error);
>             })
>         } else {
>             panic!("Problem opening the file: {:?}", error);
>         }
>     });
> }
> ```

---

### Shortcuts for Panic on Error: ```unwrap``` and ```expect``` ###

In Rust, the ```Result<T, E>``` type exposes many helper 
functions, including ones that create a shorthand for
```match```.

---

#### ```unwrap``` ####

The ```unwrap``` method is implemented identically to a
```match``` where:

* If the ```Result``` variant is ```Ok```, returns the value
  inside the ```Ok```.
* If the ```Result``` variant is ```Err```, calls the
  ```panic!``` macro.

```rust
use std::fs::File;

fn main() {
    // Either return the file handle or panic
    let greeting_file = File::open("hello.txt").unwrap();
}
```

---

#### ```expect``` ####

The ```expect``` method is similar to ```unwrap```, but it
allows us to specify an error message.

```rust
use std::fs::File;

fn main() {
    // Either return the file handle or panic with message
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}
```

---

### Propagating Errors ###

Sometimes, it is advantageous to return an error to the 
calling code instead of handling it within the function. This 
is similar to the behavior in other languages where exceptions
bubble upward until caught.

You can propagate an error by returning the ```Result``` to
the calling code.

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        // Return error to calling code
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        // Return value read from file
        Ok(_) => Ok(username),
        // Return error to calling code
        Err(e) => Err(e),
    }
}
```

---

### A Shortcut for Propagating Errors: the ```?``` Operator ###

Rust exposes a special operator [```?```] that provide a
shortcut for propagation.

This code performs the same task as the above example:

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    // If `Err`, return the error
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    // If `Err`, return the error
    username_file.read_to_string(&mut username)?;
    // Return value read from file
    Ok(username)
}
```

Under the covers, ```?``` passes the error to the ```From```
method to convert it into the error type defined in the
function signature.

---

We can shorten this further by chaining methods after the
```?```

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
```

---

Or even further by using the ```fs::read_to_string``` method
to both open and read the file.

```rust
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

---

### Where The ```?``` Operator Can Be Used

Because the ```?``` operator performs an early return of the
error type, it can only be used in a function that returns
```Result```, ```Option```, or another type that implements
the ```FromResidual``` trait.

---

### Errors in ```main()``` ###

This code will not compile, since the ```main()``` function
does not have a matching return type.

```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")?;
}
```

---

However, the ```main()``` function supports returning an 
error as well, using ```Result<(), E>``` as seen below:

```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
```

This makes the ```main``` function return ```0``` on
```Ok``` and a non-```0``` value on any ```Err```.

This is similar behavior to C and C++

---

Technically, ```main``` can return any type that implements 
the ```std::process::Termination``` trait, which contains a
function ```report``` that returns an ```ExitCode```

---
