#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width
            && self.width > rect.height
            && self.height > rect.height
            && self.height > rect.width
    }

    fn squuare(length: u32) -> Rectangle {
        Rectangle {
            width: length,
            height: length,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 50,
        height: 500,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The area of the retangle is {} square poxels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square1 = Rectangle::squuare(100);
    println!("Can square1 hold rect3? {}", square1.can_hold(&rect3));
}
