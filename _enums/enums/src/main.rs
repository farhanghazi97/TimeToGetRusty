enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from: {:?}", state);
            25
        }
    }
}

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    let penny = value_in_cents(Coin::Penny);
    let dime = value_in_cents(Coin::Dime);
    let nickel = value_in_cents(Coin::Nickel);

    println!("The value of the coin is: {penny}");
    println!("The value of the coin is: {dime}");
    println!("The value of the coin is: {nickel}");

    let alaska_quarter = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("The value of the coin is: {alaska_quarter}");

    let alabama_quarter = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("The value of the coin is: {alabama_quarter}");
}
