#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.can_hold(&rect2) {
        println!("rect1 can hold rect2");
    } else {
        println!("rect1 cannot hold rect2");
    }
}
