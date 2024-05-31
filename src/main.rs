
mod modules;
use std::io;

//use crate::ves

fn main() {
    println!("Hello, world!");

    let mut s: String = String::new();

    match io::stdin().read_line(&mut s) {

        Ok(_) => println!(""),
        Err(e) => println!("Error {}", e),
        
    }

    println!("line {}", s);
    
}
