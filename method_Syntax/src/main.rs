#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
fn main() {
    let _ret1 = Rectangle {
        width: 30,
        height: 50,
    };

    let _ret2 = Rectangle {
        width: 10,
        height: 40,
    };

    let _ret3 = Rectangle {
        width: 20,
        height: 60,
    };

    println!("Can rect1 hold rect2? {}", _ret1.can_hold(&_ret2));
    println!("Can rect1 hold rect3? {}", _ret1.can_hold(&_ret3));
}
