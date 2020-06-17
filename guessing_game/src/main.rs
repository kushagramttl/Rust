use std::io;

fn main() {
    println!("Welcome to guess the number!");
    println!("Please input the number: ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read the line!");

    println!("Your guess: {}", guess);
}
