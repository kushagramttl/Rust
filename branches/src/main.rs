fn main() {
    test_control_flow(100);
    test_control_flow(20);
    if_in_let(90);
    if_in_let(97);
}

fn test_control_flow(x: i32) {
    if x < 50 {
        println!("The value {}, is a small value", x);
    } else {
        println!("The value {}, is a large value", x);
    }
}

fn if_in_let(x: i32) {
    let num: i32 = if x % 30 == 0 {x / 5} else {x % 5};
    println!("The calculated value is: {}", num);
}
