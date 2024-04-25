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

What lifetimes are necessary is dictated by what your function
is doing.

for example, if the ```longest``` function only returned the
first parameter ```x``` then ```y``` would not require a
lifetime specification.

```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

---

When returning a reference from a function, the lifetime 
parameter for the return type needs to match the lifetime 
parameter for one of the parameters. If the reference 
returned does not refer to one of the parameters, it must 
refer to a value created within this function. However, this 
would be a dangling reference because the value will go out 
of scope at the end of the function.

```rust
// Note: This code will not compile
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str() // Returns dangling reference
}
```

```
error[E0515]: cannot return reference to local variable `result`
  --> src/main.rs:11:5
   |
11 |     result.as_str()
   |     ^^^^^^^^^^^^^^^ returns a reference to data owned by the current function
```

---

### Lifetime Annotations in Struct Definitions ###

We can define structs to hold references, but in that case we 
would need to add a lifetime annotation on every reference in 
the struct’s definition.

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```

---

### Lifetime Elision ###

This function compiled without lifetime annotations, even
though it returns a reference.

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

In early Rust versions, this would not have compiled
without lifetime annotations like the below:

```rust
fn first_word<'a>(s: &'a str) -> &'a str {
    // ...
}
```

Because this pattern (and some others) are predictable and
deterministic, they were added to Rust as *lifetime elision
rules*, allowing the borrow checker to infer the lifetimes.

---

There are three rules used to infer lifetimes when not
specified:

1. The compiler assigns a different lifetime parameter to 
   each lifetime in each input type. For example:
    * ```fn foo(x: &i32)``` gains one lifetime parameter
      and becomes ```fn foo<'a>(x: &'a i32)```
    * ```fn foo(x: &i32, y: &i32)``` gains two lifetime
      parameters and becomes
      ```foo<'a, 'b>(x: &'a i32, y: &'b i32)```
    * fn ```foo(x: &ImportantExcerpt)``` gains two lifetime
      parameters and becomes
      ```fn foo<'a, 'b>(x: &'a ImportantExcerpt<'b>)```
2. If there is exactly one input lifetime parameter, that 
   lifetime is assigned to all output lifetime parameters.
    * ```fn foo<'a>(x: &'a i32) -> &'a i32```
3. If there are multiple input lifetime parameters, but one 
   of them is ```&self``` or ```&mut self``` because this is 
   a method, the lifetime of ```self``` is assigned to all 
   output lifetime  parameters.

---

So, for the function above, the compiler does the following:

We start with no lifetimes specified

```rust
fn first_word(s: &str) -> &str { ... }
```

Rule 1 is applied (each parameter gets its own lifetime)

```rust
fn first_word<'a'>(s: &'a str) -> &str { ... }
```

Rule 2 is applied because there is only one lifetime

```rust
fn first_word(s: &str) -> &'a str { ... }
```

All requirements are met, and the compiler can proceed 
without explicit lifetime specifiers.

---

For the ```longest``` function, if we omit the lifetime
specifiers, the compiler tries to implement the same rules.

Start with no lifetimes


```rust
fn longest(x: &str, y: &str) -> &str { ... }
```

Apply rule 1


```rust
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str { ... }
```

Rule 2 cannot apply, because there is more than one input
lifetime.

Rule 3 cannot apply, because this is a function, not a
method, so none of the input parameters are ```self```

Because neither applied, the compiler cannot infer the
output lifetime, so we get an error.

---

### Lifetime Annotations in Method Definitions ###

For struct methods lifetime references are added to the
```impl``` block as well as any methods that require them.

```rust
impl<'a> ImportantExcerpt<'a> {
    // No lifetime needed on this method
    fn level(&self) -> i32 {
        3
    }

    // This method is covered by elision rule 3
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

---

### The Static Lifetime ###

The ```'static``` lifetime indicates that the affected
reference can live for the duration of the program.

Note: All string literals have the ```'static``` lifetime.

```rust
let s: &'static str = "I have a static lifetime.";
```

Be careful when reacting to an error message that suggests
using ```'static```, as this often results from a dangling
reference or mismatched lifetimes and may not require using
```'static```

---

### Generic Type Parameters, Trait Bounds, and Lifetimes Together ###

Here is an example of using generic types, traits, and
lifetimes in a single function.

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

---
