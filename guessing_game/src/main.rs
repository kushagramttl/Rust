use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to guess the number!");
    println!("Please input the number: ");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line!");

    let guess: u32 = guess.trim().parse().expect("Please enter a number");

    println!("Your guess: {}", guess);
    println!("The secret number is: {}", secret_number);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Equal => println!("Guessed right!"),
        Ordering::Greater => println!("Too big!")
    }
}
