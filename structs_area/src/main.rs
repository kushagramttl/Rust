#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    println!("The area of the Rectangle is: {} \n", area(&rect1));

    println!("Rectangle info: {:?} \n", rect1);

    println!("For a pretty output with debugging: {:#?} \n", rect1);

    println!("The area calculated using method in rectangle object: {}\n", rect1.area());

    let rect2 = Rectangle {
        height: 20,
        width: 20
    };

    let rect3 = Rectangle {
        height: 50,
        width: 10
    };

    println!("Can rect1 hold rect2: {}\n", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3: {}\n", rect1.can_hold(&rect3))
}

fn area(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}
