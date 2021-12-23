use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let _config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1)
    });
    println!("Sreaching for {}", _config.query);
    println!("In file {}", _config.filename);
    if let Err(e) = minigrep::run(_config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}