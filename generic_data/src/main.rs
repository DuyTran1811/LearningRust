struct Ponit<T, U> {
    x: T,
    y: U,
}

impl<T, U> Ponit<T, U> {
    fn mixup<V, W>(self, other: Ponit<V, W>) -> Ponit<T, W> {
        Ponit {
            x: self.x,
            y: other.y,
        }
    }
}
fn main() {
    let _p1 = Ponit { x: 5, y: 10.4 };
    let _p2 = Ponit { x: "Hello", y: 'C' };
    let _p3 = _p1.mixup(_p2);

    println!("p3.x = {}, p3.y = {}", _p3.x, _p3.y);
}
