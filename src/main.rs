use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let filename = &args[1];

    let contents = fs::read_to_string(filename)
    .expect(&format!("Could not read file '{}'", filename));

    println!("Contents read: \n'{}'", &contents);
}