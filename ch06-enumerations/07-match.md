## The ```match``` Control Flow Structure ##

In Rust control flow, the ```match``` construct allows comparison and
branching based on the values of a series of patterns (like an enum).

Patterns can include:

* Literals
* Variables
* Wildcards
* Enums (of course)
* And many other things

---

Here is an example of using ```match``` for control flow with an enum;

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let c1 = Coin::Penny;
    let c2 = Coin::Nickel;
    let c3 = Coin::Dime;
    let c4 = Coin::Quarter;

    let total = cents(c1) + cents(c2) + cents(c3) + cents(c4);

    println!("Total: {total} cents");
}

fn cents(coin: Coin) -> u8 {
    // Here, we're returning a different value for each variant of Coin
    match coin {
        // Use a curly-brace block when there is more than one instruction
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

---
