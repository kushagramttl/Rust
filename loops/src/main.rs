fn main() {
    let mut counter = 0;

    loop {
        counter += 1;

        if counter == 10 {
            println!("The counter has reached 10");
            break;
        }
        else {
            println!("Continuing");
        }
    }

    while counter > 0 {
        println!("executing while at {}", counter);
        counter -= 2;
    }

    let a = [10, 20, 30, 40, 50];
    for elem in a.iter() {
        println!("The value in iteration is: {}", elem);
    }
}
