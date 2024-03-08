## The String Type ##

Rust has two different kinds of strings:

* String literals are of type ```&str```
    * ```&str``` values must be known at compile-time and cannot be altered 
      during runtime.
    * These are stored on the stack.
* Rust also implements a complex type ```String```, which has additional
  functionality and is a good example to illustrate the ownership rules.
    * ```String``` values can be populated by the code during run-time, for
      example, when storing the value a user inputs.
    * These are stored on the heap.

**Note**: The rules for ```String``` that affect ownership apply to ***all
          complex data types***, whether provided by the language or
          user-defined.

Here are examples of converting from a literal to a ```String```

```rust
// Although these leverage different traits (a later concept), both examples
//   produce equivalent String results
let s1 = "hello".to_string();
let s2 = String::from("hello");
```

Importantly, the ```String``` type can be mutated

```rust
let mut s = String::from("hello");
s.push_str(", world!"); // Here, we're appending to the string (mutating)
println!("{s}");         // prints out 'hello, world!'
```

---
