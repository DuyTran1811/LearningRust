use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5, 6];
    println!("Please enter an array index.");

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index enter was not a number");

    let element = a[index];
    println!("The value of element at index {} is: {}", index, element);
}