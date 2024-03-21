## Variable and Data Interactions with Clone ##

The ```clone``` method in Rust provides a means of copying the data (on the
heap), not just the reference,

```rust
let s1 = String::from("hello"); // binds reference to the string to s1
let s2 = s1.clone(); // binds reference to a new string to s2

// Because the string itself has been copied, both s1 and s2 are valid
//   ... so this works fine
println!("{s1} {s2}");
```

Cloning is costly in memory (and even in CPU cycles), so we should be certain
it's necessary before cloning.

---
