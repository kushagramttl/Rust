#[derive(Debug)]

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    println!("Hello, world!");
    let val = value_in_cents(Coin::Penny);
    println!("The value of Penny is: {}", val);

    let some_value = Some(0u8);

    if let Some(3) = some_value {
        println!("The value has matched!")
    } else {
        println!("Nothing matched")
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
