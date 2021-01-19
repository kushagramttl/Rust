fn main() {
    // One way to initialize a vector is using assigning a new variable and then pushing
    // values into the vecor
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    v.iter().enumerate().for_each(|(i, arg)| {
        println!("The value for v at index {0} is {1}", i, arg);
    });

    // The second method to initialize a vector is by using a macro
    // Below is an example of how the macro can be used
    let v2: Vec<i32> = vec![2, 3, 4, 5];

    v2.iter().enumerate().for_each(|(i, arg)| {
        println!("The value for v2 at index {0} is {1}", i, arg);
    });

    vector_scoping_sample();
    print_vector_value();
}

fn vector_scoping_sample() {
    // Here we will create a vector and show that when vector is freed when its scope ends

    println!("Inside scoping sample");

    let v1: Vec<i32> = vec![1, 2, 3, 4];

    {
        let v2: Vec<i32> = vec![7, 8, 9];

        println!("We can see that v1 and v2 are both accessible here");
        v1.iter().enumerate().for_each(|(i, arg)| {
            println!("The value for v1 at index {0} is {1}", i, arg);
        });

        v2.iter().enumerate().for_each(|(i, arg)| {
            println!("The value for v2 at index {0} is {1}", i, arg);
        });
    }

    println!("Outside the scope only v1 is accessible");

    // We can notice that v2 cannot be accessed here

    v1.iter().enumerate().for_each(|(i, arg)| {
        println!("The value for v1 at index {0} is {1}", i, arg);
    });
}

fn print_vector_value() {
    // Method to demonstrate fetching value from vector
    println!("Inside printing method for vector");

    let v1: Vec<i32> = vec![1, 2, 3, 4];

    let second = &v1[1];
    println!("The second value of the vector is: {0}", second);

    // Another method to fetch values is to use the get method
    // The get method retruns the reference object
    // Using match would allow to check when index is out of range
    // get returns None if the index is not present
    match v1.get(2) {
        Some(_second) => println!("The third value is: {0}", v1[2]),
        None => println!("The value did not match the second value"),
    }
}
