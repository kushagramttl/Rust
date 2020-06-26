use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input the number: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess: {}", guess);
        println!("The secret number is: {}", secret_number);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("Guessed right!");
                break;
            },
            Ordering::Greater => println!("Too big!")
        }
    }
}
