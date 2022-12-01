#[allow(dead_code)]

fn main() {
    let penny = Coin::Penny;
    let quarter = Coin::Quarter(State::Massachusetts);

    println!("Coin value of coin {}", value_in_cents(quarter));
}

#[derive(Debug)]
enum State {
    Alabama,
    Alaska,
    Massachusetts,
}
enum Coin {
    Dime,
    Nickel,
    Penny,
    Quarter(State),
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Dime => 10,
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Quarter(state) => {
            println!("State Quarter: {:?}", state);
            25
        }
    }
}
