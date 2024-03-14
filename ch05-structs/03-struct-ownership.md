## Ownership of Struct Data ##

In general, it is best to have a struct own its own data fields. In the
previous examples, this is why the ```User``` struct was defined with
```String``` members rather than ```&str``` references.

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

If we used references instead, it would require the use of *lifetimes*, so
this would not work:

```rust
struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn main() {
    let User1 = User {
        active: true,
        // These cause the following error
        //   missing lifetime parameter
        username: "someuser",
        email: "someuser@somedomain.com",
        sign_in_count: 1,
    };
}
```

---
