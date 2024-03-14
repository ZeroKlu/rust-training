## Enums with Varying Data ##

One advantage to enums over structs in a case like this is that the data
types and number assigned do not have to be the same among variants.

```rust
#[derive(Debug)]
enum IpAddr {
    // Here, we're defining four integer octets instead of a String
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    // Here, we're assigning four integer octets instead of a String
    let home = IpAddr::V4(127, 0, 0, 1);
    println!("home = {:?}", home);

    let loopback = IpAddr::V6(String::from("::1"));
    println!("loopback = {:?}", loopback);
}
```

---

Data types can include other enums or structs as well.

Here's an example from the book that includes several different amounts
and types of data:

```rust
enum Message {
    Quit,                        // No data: Just a variant
    Move {x: i32, y: i32},       // Two named fields
    Write(String),               // One String value
    ChangeColor(i32, i32, i32),  // Three integer values
}
```

---

We could have defined several different structs, like this:

```rust
struct QuitMessage;                       // Unit struct
struct MoveMessage {                      // struct
    x: i32,
    y: i32,
}
struct WriteMessage(String);              // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
```

But it would be complicated to define a function to accept several 
different possible structs.

---

In this scenario, the enum definition is more useful, because we can 
easily define a single function that accepts our enum as an argument.

```rust
fn do_something(Message) {
    // ...
}
```

---
