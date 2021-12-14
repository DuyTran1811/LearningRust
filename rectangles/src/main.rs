#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let _rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: dbg!(50 + 10),
    };
}
