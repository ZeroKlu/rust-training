## To panic! or Not to panic! ##

When should you use ```panic!``` vs ```Result```?

---

### Examples, Prototype Code, and Tests ###

For prototypes and testing code, the best practice is to panic
when an error occurs, as this gives the developer the 
opportunity to address the issue without the code continuing.

Because they fail over to ```panic!```, both ```unwrap``` and
```expect``` are appropriate for tests and prototypes.

---

### Cases in Which You Have More Information Than the Compiler ###

When you have written code that ensures that the result state 
will be ```Ok```, but the compiler doesn't understand the 
logic, again, ```unwrap``` and ```expect``` are appropriate.

```rust
use std::net::IpAddr;

let home: IpAddr = "127.0.0.1"
    .parse()
    .expect("Hardcoded IP address should be valid");
```

We're creating an ```IpAddr``` instance by parsing a 
hardcoded string. We can see that ```127.0.0.1``` is a valid 
IP address, so it's acceptable to use ```expect``` here.

However, having a hardcoded, valid string doesn't change the 
return type of the ```parse``` method: we still get a 
```Result``` value, and the compiler will still make us 
handle the ```Result``` as if the ``Err`` variant is a 
possibility because the compiler isn't smart enough to see 
that this string is always a valid IP address.

If the IP address string came from a user rather than being 
hardcoded into the program and therefore did have a 
possibility of failure, we'd definitely want to handle the 
```Result``` in a more robust way instead.

---

### Guidelines for Error Handling ###

You should ```panic``` if the code could end up in a *bad
state* (some assumption, guarantee, contract, or invariant 
has been broken) and...

* The bad state is unexpected<br>or...
* Downstream code relies on this bad state not occurring<br>
  or...
* There's not a good way to encode this information in the types you use.

---

You should return an error when a call to your code passes 
invalid data. This ensures that the calling code can identify 
the error condition.

---

You should ```panic``` if a call to external code (outside 
your control) returns an invalid state that you cannot fix.

---

You should return a ```Result``` when an expected failure 
occurs.

---

When your code performs an operation that could put a user at 
risk if it's called using invalid values, your code should 
verify the values are valid first and panic if the values 
aren't valid.

---

### Creating Custom Types for Validation ###

In the guessing game program, we did not validate that the
user input was a number.

We could easily improve this by checking if the guess can
be parsed as an ```i32``` and then check that it is in range.

```rust
loop {
    // --snip--

    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    if guess < 1 || guess > 100 {
        println!("The secret number will be between 1 and 100.");
        continue;
    }

    match guess.cmp(&secret_number) {
        // --snip--
}
```

---

But this is a non-optimal solution, since this adds
significant overhead, especially if many functions had to implement this requirement.

An alternative is to create a custom type and include the 
validation steps in its ```new``` function.

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
```

---
