use std::env;
use std::fs;
use std::process;


fn main() {
    let args: Vec<String> = env::args().collect();
    
    let filename = &args[1];

    let contents = fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("Error reading file '{}': {}", filename, err);

        process::exit(1);
    });

    if let Err(err) = sudoku_solver::run(&contents) {
        eprintln!("Application error: {}", err);

        process::exit(1);
    }
}