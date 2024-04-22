## Validating References with Lifetimes ##

A lifetime is a form of a generic that (instead of ensuring a
data type has a necessary behavior) ensures that a reference
will remain valid for as long as is needed.

Every reference has a lifetime. Typically, these are inferred 
from the code, but when there are multiple possible inferred
lifetimes, we must explicitly annotate them.

---

### Preventing Dangling References with Lifetimes ###

Consider this unsafe code:

```rust
// Note: This code will not compile
fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    // x is out of scope, so this is invalid
    println!("r: {}", r);
}
```

```
error[E0597]: `x` does not live long enough
 --> src/main.rs:6:13
  |
6 |         r = &x;
  |             ^^ borrowed value does not live long enough
7 |     }
  |     - `x` dropped here while still borrowed
8 |
9 |     println!("r: {}", r);
  |                       - borrow later used here
```

The problem here is that ```r``` outlives ```x```. However,
since its value is a reference to ```x``` we would have a
dangling reference if this compiled.

---

### The Borrow Checker Ensures Data Outlives Its References ###

Consider the same code with the lifetimes indicated in the
comments (```r``` as ```'a``` and ```x``` as ```'b```):

```rust
// Note: This code will not compile
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
```

We need to ensure that the data in ```x``` outlives its 
reference, ```r```, which we can accomplish by bringing
```x``` into the outer scope.

```rust
fn main() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+
```

---

### Generic Lifetimes in Functions ###

Assume we have this code and need to implement the
```longest``` function...

```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
```

We want the function to take string slices, which are 
references, rather than strings, because we don’t want the 
function to take ownership of its parameters.

We might try to implement it like this:

```rust
// Note: This code will not compile
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

```
error[E0106]: missing lifetime specifier
 --> src/main.rs:9:33
  |
9 | fn longest(x: &str, y: &str) -> &str {
  |               ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
help: consider introducing a named lifetime parameter
  |
9 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  |           ++++     ++          ++          ++
```

Because the compiler cannot identify whether the returned
```&str``` slice refers to ```x``` or ```y```, it is unable
to infer the lifetime for the borrow checker.

---

### Lifetime Annotation Syntax ###

Lifetime annotations have a slightly unusual syntax: the 
names of lifetime parameters must start with an apostrophe 
(```'```) and are usually all lowercase and very short, like 
generic types. Most people use the name ```'a``` for the 
first lifetime annotation. We place lifetime parameter 
annotations after the ```&``` of a reference, using a space 
to separate the annotation from the reference'’'s type.

Here are some examples: a reference to an ```i32``` without a 
lifetime parameter, a reference to an ```i32``` that has a 
lifetime parameter named ```'a```, and a mutable reference to 
an ```i32``` that also has the lifetime ```'a```.

```rust
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

---

### Lifetime Annotations in Function Signatures ###

To use lifetime annotations in function signatures, we need 
to declare the generic lifetime parameters inside angle 
brackets between the function name and the parameter list, 
just as we did with generic type parameters.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

This defines that for a lifetime ```'a```, the function
accepts two ```&str``` parameters whose lifetimes are at least
```'a``` long.

This will now work with our original ```main``` function.

Important! The annotations do not change the lifetimes of the
references. They just tell the compiler how to interpret the
lifetimes of the function.

---

As long as the references live long enough, the function can
be successfully called from ```main```

```rust
fn main() {
    // Example 1 works
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
```

Output:
```
The longest string is abcd
```

---

```rust
fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}
```

Output:
```
The longest string is long string is long
```

---

But if the references do not meet the expected lifetime
from the annotated function, we receive an error.

```rust
// Note: This code will not compile
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
```

```
error[E0597]: `string2` does not live long enough
 --> src/main.rs:6:44
  |
6 |         result = longest(string1.as_str(), string2.as_str());
  |                                            ^^^^^^^^^^^^^^^^ borrowed value does not live long enough
7 |     }
  |     - `string2` dropped here while still borrowed
8 |     println!("The longest string is {}", result);
  |                                          ------ borrow later used here
```

---

### Thinking in Terms of Lifetimes ###



