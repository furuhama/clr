extern crate csv;
extern crate encoding_rs;

use std::fs::File;
use std::io::{self, BufReader, Read};
use std::env;
use std::error::Error;
use csv::StringRecord;
use encoding_rs::SHIFT_JIS;

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
    // Read input data, and put them all on buffer at once.
    // (If an input data size gets bigger,
    // this code may cause performance problem.
    // It should be refactored to processing data as a stream.)
    let mut buf = Vec::new();

    // If an argument is given,
    // it will be taken as a filename of target CSV file.
    // Or, it will be taken as an input data ig given from STDIN.
    if let Some(filename) = env::args().nth(1) {
        let file = File::open(filename)?;
        let mut buf_reader = BufReader::new(file);

        buf_reader.read_to_end(&mut buf)?;
    } else {
        let mut input = io::stdin();

        input.read_to_end(&mut buf)?;
    };

    // Even if input `buf` variable data encoding is `UTF-8` already,
    // it does not raise an error on the decoding process written below.
    let (buf_as_utf8, _, _) = SHIFT_JIS.decode(&buf);

    let mut rdr = csv::Reader::from_reader(buf_as_utf8.as_bytes());

    let header = rdr.headers()?;
    colorize_row(&header);

    for record in rdr.records() {
        let record = record?;
        colorize_row(&record);
    }

    Ok(())
}
