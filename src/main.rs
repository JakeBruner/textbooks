extern crate textbooks;

use std::env;

use textbooks::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    if let Err(e) = textbooks::run(config) {
        println!("Error: {}", e);
        std::process::exit(1);
    }
}
