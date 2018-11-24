extern crate csv;

use std::fs::File;
use std::io::BufReader;
use std::env;
use std::error::Error;

const ANSI_COLOR_CODES: [&str; 8] = [
    "\x1b[31m", // Red
    "\x1b[32m", // Green
    "\x1b[33m", // Yellow
    "\x1b[34m", // Blue
    "\x1b[35m", // Magenta
    "\x1b[36m", // Cyan
    "\x1b[37m", // White
    "\x1b[0m",  // Reset all attributes
];

fn main() -> Result<(), Box<Error>> {
    let filename = env::args().nth(1).unwrap();
    let file = File::open(filename)?;
    let buf_reader = BufReader::new(file);
    let mut rdr = csv::Reader::from_reader(buf_reader);

    for record in rdr.records() {
        let record = record?;
        println!("{:?}", record);
    }

    for color in ANSI_COLOR_CODES.iter() {
        println!("{}test text", color);
    }

    Ok(())
}
