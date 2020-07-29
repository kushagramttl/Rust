fn main() {
    let s1 = String::from("Hello World");
    let s2 = s1;
    // Here we cannot print s1 because once the value of s2 is assigned, s1 will be 'moved' to s2
    // s1 will then be no longer valid and printing it will give a compilation error
    println!("The string is: {}", s2);

    let s3 = String::from("Shallow copy from s3 to s4");
    let s4 = s3.clone();

    // Here s3 and s4 will remain valid
    // A deep copy of s3 is created and assigned to s4
    // This way both s3 and s4 remain valid
    println!("Value in s3: {}", s3);
    println!("Value in s4: {}", s4);

    let mut mutable_string = String::from("Mutable String");
    println!("Original string: {}", mutable_string);
    update_string(&mut mutable_string);
    println!("Updated string: {}", mutable_string);

    let borrow_string = String::from("Hello World!");

    println!("Borrowed String is: {}", borrow_string);

    let word = first_word(&borrow_string);

    println!("The first word is: {}", word)

}

fn update_string(some_string: &mut String) {
    some_string.push_str(", with pushed content");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}