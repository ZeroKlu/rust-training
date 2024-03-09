## Ownership with Functions ##

Ownership transfers to and from functions in much the same way as
variables.

### Passing variables to functions ###

* If a value requires allocation, then passing it to a function transfers
  ownership to the function, and the original variable becomes invalid.
* If the value does not require allocation (stack-only values), then it is
  copied into the function.

```rust
fn main() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s);            // The value for s moves into the function
                                   // Since the value has moved, s is no longer valid
    
    let x = 5;                     // x comes into scope

    makes_copy(x);                 // Since x is i32, it is copied, not moved, so it remains valid
} // s and x both go out of scope, but s doesn't need to be dropped, because it already moved

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");            // some_string is valid, so I can use it here
} // some_string falls out of scope - drop is called, since it is owned by 
  // the function (memory is freed)

fn makes_copy(some_integer: i32) {        // some_integer comes into scope
    println!("{some_integer}");            // some_integer is valid, so I can use it here
} // some_integer falls out of scope - nothing special happens, since it 
  // didn't require allocation

```

---

### Returning Values from Functions ###

* When a function creates and returns a value, the value is owned by the
  variable that captures the return. When the function ends, and its variable
  goes out of scope, nothing happens, because ownership has moved.
* If a function receives a variable and then returns the same variable,
  ownership transfers twice: once to the function when it is called and once
  to the variable that captures the return value.

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return to s1 (s1 is valid)

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into takes_and_gives_back (s2 is not valid)
                                        // then, takes_and_gives_back moves its return to s3 (s3 is valid)

    println!("s1 = {s1} : s3 = {s3}");  // s1 and s3 are valid, so this is fine

    // s2 is invalid, so this won't work
    // println!("s2 = {s2}");

} // s1 and s3 go out of scope and are dropped. s2 was already moved, so nothing happens with it

fn gives_ownership() -> String {
    let some_string = String::from("yours");    // some_string comes into scope
    some_string                                 // some_string is returned and moved into s1
} // some_string goes out of scope but was already moved so nothing happens

fn takes_and_gives_back(a_string: String) -> String {  // a_string comes into scope
    a_string                                           // a_string is moved to s3
} // a_string goes out of scope but was already moved so nothing happens

```

---

### Returning Multiple Values ###

One way that we cna work around the problem of passing ownership along with
a function argument is to return the original variable along with the
computed result.

```rust
fn main() {
    let s1 = String::from("hello");
    // We can work around the problem of passing ownership in some cases
    //   by returning both the computed result and the original value
    let (s2, len) = calculate_length(s1);
    println!("Length of '{s2}' is {len} characters.");
}

// Rust allows returning multiple values (in a tuple)
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
```

---

But this still doesn't solve the problem that sometimes we want a function
to use a variable but not take away ownership, for example, if we need to
pass the variable repeatedly within a recursive function.
