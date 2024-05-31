## Controlling How Tests Are Run ##

When you execute ```cargo test```, cargo compiles a test binary. It then
(by default) runs all tests in parallel and captures any output (does not
display output).

You can add command-line arguments to control how tests are run. There are
two separate types of command-line arguments: ones that control
```cargo test``` itself and ones that are passed to the binary. These are 
separated by a double-dash [```--```].

```
cargo test [cargo args] -- [binary args]
```

To access a list of the cargo arguments, run this:

```
cargo test --help
```

To access a list of the binary arguments, run this:

```
cargo test -- --help
```

### Running Tests in Parallel or Consecutively ###

When you run multiple tests, by default they run in parallel using 
threads, meaning they finish running faster and you get feedback quicker. 
Because the tests are running at the same time, you must make sure your 
tests don’t depend on each other or on any shared state, including a 
shared environment, such as the current working directory or environment 
variables.

This can be a problem if multiple tests require access to the same resource
(for example, if multiple tests write to the same file).

In a case like this, you can instruct the test to run using only a single
thread (which forces the tests to run sequentially instead of parallel) 
like this:

```
cargo test -- --test-threads=1
```

This makes the test take longer, but it will prevent race conditions 
accessing resources.

---

### Showing Function Output ###

Ordinarily, ```cargo test``` captures all output if a test passes, meaning
we cannot see the results of the code (like ```println!()``` calls).

Consider the following code:

```rust
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}
```

When ```this_test_will_fail()``` fails, we receive a detailed response
indicating that the test failed because the compared value ```5``` does
not match the returned value ```10```.

```
---- tests::this_test_will_fail stdout ----
I got the value 8
thread 'tests::this_test_will_fail' panicked at src\lib.rs:19:9:
assertion `left == right` failed
  left: 5
 right: 10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Importantly, that report, shows the ```println!()``` result: 
"```I got the value 8```"

But for ```this_test_will_pass()```, we only see the result:

```
test tests::this_test_will_pass ... ok
```

In some cases, it's useful, even for a passing test to see the output
(in this example, "```I got the value 4```").

We can accomplish this by adding the ```show-output``` switch like this:

```
cargo test -- --show-output
```

Doing that, the report now shows the standard output from the passed test:

```
successes:

---- tests::this_test_will_pass stdout ----
I got the value 4
```

---

### Running a Subset of Tests by Name ###

Because tests can take time to run, it is sometimes advisable to run only
some (or only **one**) of the tests in a library.

Consider the following:

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}
```

#### Specifying One Test ####

Here, we have three tests set up. But we can choose to run just one of them
by passing the test function name, like this:

```cargo test one_hundred```

Which only runs the identified test (but identifies that two others were
filtered out):

```
running 1 test
test tests::one_hundred ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s
```

#### Filtering to Run Multiple Tests ####

We can also specify a shared part of the names of multiple tests, like the
```add``` part of ```add_three_and_two``` and ```add_two_and_two```

```
cargo test add
```

```
running 2 tests
test tests::add_three_and_two ... ok
test tests::add_two_and_two ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s
```

---

### Ignoring Some Tests Unless Specifically Requested ###

Sometimes a given test is known to be long-running or for other reasons
(unavailable resource, e.g.) should not be run except when specified.

For cases like this, we can indicate with the ```#[ignore]```
decoration that the test should be ignored.

Consider the following:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
        assert_eq!(2 + 2, 4);
    }
}
```

Now, if we run the standard ```cargo test```, we see this:

```
running 2 tests
test tests::expensive_test ... ignored
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests ignore

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

But if we apply the ```--ignored``` switch, we can force the marked
test(s) to be executed instead.

```
cargo test -- --ignored
```

```
running 1 test
test tests::expensive_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

   Doc-tests ignore

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

---

Or we can use ```--include-ignored``` to run all the tests, whether or not
they are ignored.

```
cargo test -- --include-ignored
```

```
running 2 tests
test tests::expensive_test ... ok
test tests::it_works ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests ignore

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

---
