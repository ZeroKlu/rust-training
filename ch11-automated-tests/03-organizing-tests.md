## Test Organization ##

There are two types of testing to consider:

* Unity Testing
    * Small, focused tests targeting a single module in isolation
    * Can test private interfaces
* Integration Testing
    * External to library (use library as external code would)
    * Only tests public interfaces
    * Can test multiple modules

---

### Unit Tests ###

Unit tests are used to quickly pinpoint where code is and is'nt working
correctly.

The convention is to:
1. Create a module names ```tests``` in each file along with the code to 
   be tested.
2. Annotate the module with ```cfg(test)```

---

#### The Tests Module and ```#[cfg(test)]``` ####

Using the ```#[cfg(test)]``` annotation ensures that the test code is
executes only when you run ```cargo test``` and not ```cargo build```

From the earlier adder exercise, recall the following:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
```

In this code, the ```cfg``` attribute stands for *configuration* and tells
the compiler to consider the annotated code ony given a specific 
configuration option (in this case, when compiling for testing under
```cargo test```)

Even helper functions in the annotated module are ignored when not testing,
not just the functions marked with ```#[test]```

This saves time when you want to just build the code and not execute the
tests.

---

#### Testing Private Functions ####

In many languages, it is impossible to test private functions, and there is
a vocal sect of the testing community that believes that private functions
should not be tested directly. Nevertheless, Rust supports doing so.

Consider the following example with a private function:

```rust
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

// Private function (not marked with `pub`)
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    // Bring all parent functionality into scope
    use super::*;

    #[test]
    fn internal() {
        // Allows us to test `internal_adder()`
        assert_eq!(4, internal_adder(2, 2));
    }
}
```

---

### Integration Tests ###

Consider the following example:

In /src/lib.rs

```rust
pub fn add(left: usize, right: usize) -> usize {
    left + right
}
```

Create a directory called ```tests``` containing a new file
```integration_tests.rs``` so that the tree looks like this:

```
external
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs
```

In /tests/integration_test.rs

```rust
use external;

#[test]
fn add_two() {
    assert_eq!(4, external::add(2, 2));
}
```

---

After this, running ```cargo test``` looks like this:

```
     Running unittests src\lib.rs (target\debug\deps\external-3220d6f6f862afb8.exe)
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests\integration_test.rs (target\debug\deps\integration_test-9d0eff30c914a834.exe)

running 1 test
test add_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests external

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

---

We can limit the results to only the test(s) in the /tests by adding
the ```--test <filename>``` switch, like this:

```
cargo test --test integration_test
```

```
Running tests\integration_test.rs (target\debug\deps\integration_test-9d0eff30c914a834.exe)

running 1 test
test add_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

---

#### Submodules in Integration Tests ####

You can add multiple files in the /tests directory

We can create a new file ```common.rs```

```
external
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── common.rs
    └── integration_test.rs
```

Or create an additional folder ```common``` containing a file ```mod.rs```

```
external
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── common
    │   └── mod.rs
    └── integration_test.rs
```

We can now add the content of ```/common``` to the test

In /tests/integration_test.rs

```rust
use external;

// Bring the added content into scope
mod common;

#[test]
fn add_two() {
    assert_eq!(4, external::add(2, 2));
}
```

---

### Integration Tests for Binary Crates ###

In a binary crate, we cannot create an external integration tests file
