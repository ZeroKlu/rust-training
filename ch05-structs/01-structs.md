## Using Structs to Structure Related Data ##

A ```struct``` (short for 'structure') is a custom data type that permits
grouping related data together meaningfully.

Structs are similar to tuples in that they can contain multiple values. Where they differ is that in a struct, you name each element, so that they can be
accessed in a meaningful way.

---

### Defining and Instantiating a Struct ###

We define a struct as follows, using the ```struct``` keyword and pairs of
```variable: type``` declarations.

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}
```

We instantiate a struct as follows, using the assigned type name and
pairs of ```variable: value``` elements.

```rust
fn main() {
    let my_user = User {
        active: true,
        username: String::from("someuser"),
        email: String::from("someuser@somedomain.com"),
        sign_in_count: 0
    };
}

struct User {
    // * SNIP *
}
```

We access the struct's member variables using dot-syntax as follows:

```rust
fn main() {
    let my_user = User {
        // * SNIP *
    }

    println!("{}", my_user.email);
}

struct User {
    // * SNIP *
}
```

We can declare a struct as mutable and modify values, but this makes the
entire instance mutable, not just a desired member.

```rust
fn main() {
    let mut my_user = User {
        // * SNIP *
    }

    my_user.email = String::from("some.user@somedomain.com");
}

struct User {
    // * SNIP *
}
```

A typical pattern when using structs is to create a function that builds
the struct instance, like this:

```rust
fn main() {
    let my_user = build_user("anotheruser", "anotheruser@somedomain.com");

    // ...
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}
```

And we can improve this by using *field init shorthand*. This allows
parameters whose names match the field names in the struct to be
initialized with simplified syntax, like this:

```rust
fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        // Note how we can omit the value when the field names match
        username,
        email,
        sign_in_count: 1
    }
}
```

---

### Creating Instances from Other Instances with Struct Update Syntax ###

A common need is to duplicate most of an existing struct instance while 
modifying some values.

This is how that would look with our current knowledge set

```rust
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someuser1"),
        email: String::from("someuser1@somedomain.com"),
        sign_in_count: 1
    }

    let user2 = User {
        active: user1.active,
        // Only the username differs from user1
        username: String::from("someuser2"),
        email: user1.email,
        sign_in_count: user1.sign_in_count
    }
}
```

That's a lot of redundant code, so Rust provides a special syntax for
struct updates.

```rust
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someuser1"),
        email: String::from("someuser1@somedomain.com"),
        sign_in_count: 1
    }

    let user2 = User {
        // Only the username differs from user1
        username: String::from("someuser2"),
        // The update syntax gets the rest of the fields from user1
        ..user1
    }
}
```

Note: ```user1``` is invalid following this update, because the String
```user1.email``` is moved to the new instance.

If we had set both String variables manually and only allowed the update
to set ```active``` and ```sign_in_count```, then ```user1``` would
remain valid.

---


