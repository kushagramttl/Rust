#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    println!("The area of the Rectangle is: {}", area(&rect1));

    println!("Rectangle info: {:?}", rect1);

    println!("For a pretty output with debugging: {:#?}", rect1);
}

fn area(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}
