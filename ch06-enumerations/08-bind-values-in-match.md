## Patterns That Bind to Values ##

We can bind values to enums and use them in our match construct

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    // Binding US State to Quarter
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    // * SNIP *
}

fn main() {
    let c1 = Coin::Penny;
    let c2 = Coin::Nickel;
    let c3 = Coin::Dime;
    // Binding Alaska to c4
    let c4 = Coin::Quarter(UsState::Alaska);

    let total = cents(c1) + cents(c2) + cents(c3) + cents(c4);

    println!("Total: {total} cents");
}

fn cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // Accessing the value bound to the Quarter variant
        Coin::Quarter(state) => {
            println!("State Quarter: {:?}", state);
            25
        },
    }
}
```

---
