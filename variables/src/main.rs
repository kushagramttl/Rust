const MAX_VAL: u32 = 50_000;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    println!("The value of MAX_VAL is: {}", MAX_VAL);

    let tup: (i32, f64, u8) = (500, 32.2, 8);
    let (x, y, z) = tup;

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    println!("The value for x using . operator is: {}", tup.0);
    println!("The value for y using . operator is: {}", tup.1);

    let arr = ["The", "Array", "Test"];
    
    let arr2 = [3;5];

    println!("The 2nd value in arr is: {}", arr[1]);

    println!("The 2nd value in arr2 is: {}", arr2[1]);
}
