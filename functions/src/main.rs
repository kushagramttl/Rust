fn main() {
    println!("Hello, world!");
    dummy_function(40);
    let x: i32 = test_return_value(30);
    println!("The returned value is: {}", x);
}

// A dummy function that will print a numerical value as per calculation
fn dummy_function(x: i32) {
    println!("The value of x is: {}", x);

    // To update the value and learn about scoping of variables
    let y = {
        let x = 50;
        x + 2
    };

    println!("Value of y is: {}", y);
}

// Function to test the return value of the function
fn test_return_value(x: i32) -> i32 {
    x + 2
}
