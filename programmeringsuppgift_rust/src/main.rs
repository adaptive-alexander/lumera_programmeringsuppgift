extern crate core;

mod payment_file_handler;
mod tests;

use crate::payment_file_handler::payment_file_handler;

use std::io::stdin;
use std::path::PathBuf;
use std::time::Instant;

// Function to accept user input
pub fn read_input() -> String {
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");
    input
}

// Driver code
fn main() {
    loop {
        println!("Enter a file to parse:\n");
        let path_s = read_input();

        // Path buffer to pass
        let path = PathBuf::from(path_s.trim());

        // Timing the function
        let start = Instant::now();

        // Run parser
        payment_file_handler(path);
        println!("Time taken: {} µs", start.elapsed().as_micros())
    }
}
