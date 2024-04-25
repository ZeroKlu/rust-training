## Writing Automated Tests ##

Rust's borrow checker does a lot of heavy lifting to ensure
that the code is safe when compiled.

However, it does not validate that the code acts as desired.

For example, if we wrote this function...

```rust
fn square(x: i32) -> i32 {
    x * x
}
```

... Rust will make sure that there is no place in the code
where (for example) we are passing a ```String``` to the
```square``` function.

But if we modified the code to this...

```rust
fn square(x: i32) -> i32 {
    x * x * x
}
```

... Rust would not identify that this is returning the cube of
```x```, not its square.

The function passes the borrow checker, but it does not 
perform the task we intended.

This is where creating and running tests is vital to 
programming. We need a way to validate not just the syntax and
memory safety, but the logic in our programs.

---

### How to Write Tests ###

In a test function, we typically perform three actions:

1. Set up any needed data or state
2. Run the code you want to test
3. Assert the results are what you expect

For this, Rust provides several features:

* The ```test``` attribute
* The ```should_panic``` attribute
* A collection of testing macros

---

### Anatomy of a Test Function ###

A test in Rust is a function annotated with the ```test```
attribute.

1. Annotate the function with ```#[test]```
   ```rust
   #[test]
   fn my_test() {...}
   ```
2. Run the code using
   ```
   cargo test
   ```

Note: When you create a library project in Cargo, a test 
module is automatically created, including the following 
function:

```rust
#[test] // Indicates that the function is a test
fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4); // Asserts that the result is 4
}
```

If we execute ```cargo test```, we receive the following:

```
cargo test                                                  Compiling adder v0.1.0 (D:\~path~\adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.68s
     Running unittests src\lib.rs (target\debug\deps\adder-65547da1e33be00c.exe)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

---

We can create a test that fails, like this:

```rust
#[test]
fn another() {
    panic!("Make this test fail");
}
```

If we execute ```cargo test```, we receive the following:

```
cargo test                                                  Compiling adder v0.1.0 (D:\~path~\adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.48s
     Running unittests src\lib.rs (target\debug\deps\adder-65547da1e33be00c.exe)

running 2 tests
test tests::it_works ... ok
test tests::another ... FAILED

failures:

---- tests::another stdout ----
thread 'tests::another' panicked at 'Make this test fail', src\lib.rs:17:9   
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::another

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```

### Checking Results with the ```assert!``` Macro ###

In a test, we provide an argument that resolves to a bool to
the ```assert!``` macro, which evaluates the condition and
performs the following branched actions:

* On ```true```
    * Do nothing - Test PASSES
* On ```false```
    * Call ```panic!``` - Test FAILS

Let's add the following to lib.rs

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

Then inside ```mod tests```, add this:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }
}
```

Result of ```cargo test```:

```
cargo test
Compiling adder v0.1.0 (D:\~path~\adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.32s
     Running unittests src\lib.rs (target\debug\deps\adder-65547da1e33be00c.exe)

running 1 test
test tests::larger_can_hold_smaller ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

The test PASSED

---

Now, in ```mod tests```, add another test

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        // --snip--
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
}
```

```
...
running 2 tests
test tests::larger_can_hold_smaller ... ok
test tests::smaller_cannot_hold_larger ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
...
```

---

Now, we'll introduce a bug...

Modify the ```can_hold``` function to this:

```rust
// --snip--
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width < other.width && self.height > other.height
    }
}
```

```
...
running 2 tests
test tests::smaller_cannot_hold_larger ... ok
test tests::larger_can_hold_smaller ... FAILED
...
```

---

### Testing Equality with the ```assert_eq!``` and ```assert_ne!``` Macros ###

Because equal and not-equal are such common comparisons,
Rust includes special macros to handle these scenarios.

These two functions are equivalent:

```rust
#[test]
fn it_adds_two() {
    assert!(4 == add_two(2));
}
```

```rust
#[test]
fn it_adds_two() {
    assert_eq!(4, add_two(2));
}
```

As are these:

```rust
#[test]
fn it_adds_two() {
    assert!(5 != add_two(2));
}
```

```rust
#[test]
fn it_adds_two() {
    assert_ne!(5, add_two(2));
}
```

---

### Adding Custom Failure Messages ###

You can also add a custom message to be printed with the 
failure message as optional arguments to the ```assert!```, 
```assert_eq!```, and ```assert_ne!``` macros.

Consider this code:

```rust
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }
}
```

This test PASSES

---

But if we had a bug that ignored the ```name``` argument, 
like this:

```rust
pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}
```

... the test FAILS with this error:

```
running 1 test
test tests::greeting_contains_name ... FAILED

failures:

---- tests::greeting_contains_name stdout ----
thread 'tests::greeting_contains_name' panicked at 'assertion failed: result.contains(\"Carol\")', src/lib.rs:12:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::greeting_contains_name

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

---

This is less informative than it could be. We can improve this
by adding a custom error message, like this:

```rust
#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        // The extra arguments are wrapped in `format!`
        "Greeting did not contain name, value was `{}`",
        result
    );
}
```

Now, the message when we fail is more informative:

```
...
failures:

---- tests::greeting_contains_name stdout ----
thread 'tests::greeting_contains_name' panicked at 'Greeting did not contain name, value was `Hello!`', src/lib.rs:12:9
...
```

---

### Checking for Panics with ```should_panic``` ###

When we use the ```panic!``` macro to respond to specific
conditions, we can test for scenarios where we expect the 
code to panic, with the ```should_panic``` attribute, for
example:

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

```
...
running 1 test
test tests::greater_than_100 - should panic ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
...
```

---

If the code fails to panic when it should, as when we
introduce a bug that skips checks for > 100...

```rust
// --snip--
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}
```

```
...
running 1 test
test tests::greater_than_100 - should panic ... FAILED

failures:

---- tests::greater_than_100 stdout ----
note: test did not panic as expected
...
```

---

Beware: ```should_panic``` will PASS when the code panics for 
a reason other than the on expected.

To help ensure that we know that the panic was expected, we
can

```rust
// --snip--

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

---

### Using ```Result<T, E>``` in Tests ###

We can create tests that use a ```Result``` instead of 
panicking on failure, like this:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
```

Note: This can't be used in combination with 
```should_panic```.

---
