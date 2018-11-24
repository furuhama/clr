extern crate csv;

use std::fs::File;
use std::io::BufReader;
use std::env;
use std::error::Error;
use csv::StringRecord;

const ANSI_COLOR_CODES: [&str; 7] = [
    "\x1b[31m", // Red
    "\x1b[32m", // Green
    "\x1b[33m", // Yellow
    "\x1b[34m", // Blue
    "\x1b[35m", // Magenta
    "\x1b[36m", // Cyan
    "\x1b[37m", // White
];

const RESET_CODE: &str = "\x1b[0m";

fn colorize_row(row: &StringRecord) {
    for (idx, elem) in row.iter().enumerate() {
        let color_code = ANSI_COLOR_CODES[idx%ANSI_COLOR_CODES.len()];
        print!("{}{} ", color_code, elem);
    }

    println!("{}", RESET_CODE);
}

fn main() -> Result<(), Box<Error>> {
    let filename = env::args().nth(1).unwrap();
    let file = File::open(filename)?;
    let buf_reader = BufReader::new(file);
    let mut rdr = csv::Reader::from_reader(buf_reader);

    let header = rdr.headers()?;
    colorize_row(&header);

    for record in rdr.records() {
        let record = record?;
        colorize_row(&record);
    }

    Ok(())
}
