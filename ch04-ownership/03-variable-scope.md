## Variable Scope ##

Scope refers to the range within the code where an item is valid.

* When a value comes into scope it is immediately valid
* The value remains valid until it goes out of scope

Consider this example for the scope of a variable ```s``` containing a string
literal.

```rust
{
                         // s is not valid yes, since it's not declared
    let s = "hello";     // from this point forward, s is valid
    // do things with s
}                        // after the block (scope) ends, s is no longer valid 
```

So far, this matches most programming languages' concept of scope. However,
this only illustrates storing known-size values on the stack.

---
