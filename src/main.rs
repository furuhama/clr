extern crate csv;
extern crate encoding_rs;

use csv::StringRecord;
use encoding_rs::SHIFT_JIS;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufReader, Read};

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
const LINE_NUM_COLOR: &str = "\x1b[2m"; // Dim

fn print_row(row: &StringRecord, line_num: Option<usize>, width: usize, color: bool) {
    if width > 0 {
        if color {
            if let Some(n) = line_num {
                print!("{}{:>width$} | {}", LINE_NUM_COLOR, n, RESET_CODE, width = width);
            } else {
                print!("{}{:>width$} | {}", LINE_NUM_COLOR, "#", RESET_CODE, width = width);
            }
        } else {
            let label = line_num.map(|n| n.to_string()).unwrap_or_else(|| "#".to_string());
            print!("{:>width$} | ", label, width = width);
        }
    }

    for (idx, elem) in row.iter().enumerate() {
        if color {
            let color_code = ANSI_COLOR_CODES[idx % ANSI_COLOR_CODES.len()];
            print!("{}{} ", color_code, elem);
        } else {
            if idx > 0 {
                print!(" ");
            }
            print!("{}", elem);
        }
    }

    if color {
        println!("{}", RESET_CODE);
    } else {
        println!();
    }
}

struct Args {
    filename: Option<String>,
    show_line_numbers: bool,
    no_color: bool,
}

fn parse_args() -> Args {
    let mut filename = None;
    let mut show_line_numbers = false;
    let mut no_color = false;

    for arg in env::args().skip(1) {
        match arg.as_str() {
            "-n" | "--line-number" => show_line_numbers = true,
            "-C" | "--no-color" => no_color = true,
            s if s.starts_with('-') => {}
            _ => filename = Some(arg),
        }
    }

    Args { filename, show_line_numbers, no_color }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = parse_args();
    let mut buf = Vec::new();

    if let Some(filename) = args.filename {
        let file = File::open(filename)?;
        let mut buf_reader = BufReader::new(file);
        buf_reader.read_to_end(&mut buf)?;
    } else {
        let mut input = io::stdin();
        input.read_to_end(&mut buf)?;
    };

    let (buf_as_utf8, _, _) = SHIFT_JIS.decode(&buf);

    let mut rdr = csv::Reader::from_reader(buf_as_utf8.as_bytes());

    let color = !args.no_color;

    if args.show_line_numbers {
        let records: Vec<StringRecord> = rdr.records().collect::<Result<_, _>>()?;
        let width = records.len().to_string().len().max(1);

        let header = rdr.headers()?;
        print_row(&header, None, width, color);

        for (i, record) in records.iter().enumerate() {
            print_row(record, Some(i + 1), width, color);
        }
    } else {
        let header = rdr.headers()?;
        print_row(&header, None, 0, color);

        for record in rdr.records() {
            let record = record?;
            print_row(&record, None, 0, color);
        }
    }

    Ok(())
}
