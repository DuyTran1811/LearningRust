use std::io;

fn main() {
    loop {
        println!("Enter number!");
        let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");
        let _number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if _number % 4 == 0 {
            println!("Number is divisible by 4");
            break;
        } else if _number % 3 == 0 {
            println!("Number is divisible by 3");
            break;
        } else if _number % 2 == 0 {
            println!("Number is divisible by 2");
            break;
        } else {
            println!("Number is not divisible number");
        }
    }
}
