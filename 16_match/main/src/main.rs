enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter() => {
            println!("Lucky quarter");
            25
        }
    }
}

fn main() {
    let coin = Coin::Quarter;
    // println!("Value in cents: {}", value_in_cents(coin));

    // for Coin::Quarter()
    println!("Value in cents: {}", value_in_cents(coin()));
}
