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
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
