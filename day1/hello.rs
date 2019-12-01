#![allow(unused)]
#![allow(unused_imports)]

use std::io::{self, Write};
use std::fs::File;
use std::str::FromStr;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "day1/day1.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    let mut total = 0;
    for line in reader.lines() {
        let line = line.unwrap(); // Ignore errors.
        // Show the line and its number.
        println!("{}", line);
        
        fn rec_fuel(x : i32) -> i32 {
            let mut fuel = (x / 3) - 2;
            if fuel > 0 { return fuel + rec_fuel(fuel); }
            else { return 0; }
        }
        total += rec_fuel(i32::from_str(&line).unwrap());
    }

    print!("Hello {}", total);
}
