## Stack-Only Data: Copy ##

When variables are stored on the stack (for example, integers, whose size is
known at compile-time), assigning one to another results in a ```copy``` rather than a ```move```

For example:

```rust
// x and y are integers, so neither is stored on the heap, therefore...
let x = 5; // x is valid
let y = x; // y is valid (and x is still valid)

// This line is fine, because both x and y are valid
println!("{}", x + y);
```

Rust provides a concept called a *trait* (explained in a later lesson), that
applies specific functionality to a data type.

* Any type that has the trait ```Copy``` assigned will behave as above.
* Any type that has the trait ```Drop``` assigned will not permit annotating
  with ```Copy```
    * This is also true with types that contain members that implement
      ```Drop```

In general, any simple scalar types (but no types that require allocation) can
implement ```Copy```

By default, the following types implement ```Copy```

* All integer types (like ```i32```)
* All floating-point types (like ```f64```)
* Boolean (```bool```)
* Characters (```char```)
* Tuples that only contain types that themselves implement ```Copy```
    * i.e.: ```(i32, i32)``` but not ```(i32, String)```
