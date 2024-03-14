## Enumerations ##

Enumerations (or *enums*) define a type by identifying (enumerating) all of
its possible variants.

There are a number of useful built-in enums, and in addition you can define your own in your code.

Let's look at an example where we're enumerating the IP Address versions.

```rust
// Define the enumeration
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    // Assign the enum to a value
    let four = IpAddrKind::V4;

    // Call a function with a variable containing an enum value
    route(four);
    
    // Call a function with the enum value itself
    route(IpAddrKind::V6);
}

// A function that accepts an enum value
fn route(ip_kind: IpAddrKind) {
    // Example
}
```