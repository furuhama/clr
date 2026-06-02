extern crate csv;
extern crate encoding_rs;

use csv::StringRecord;
use encoding_rs::SHIFT_JIS;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};

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

fn print_row(
    w: &mut impl Write,
    row: &StringRecord,
    line_num: Option<usize>,
    width: usize,
    color: bool,
) {
    if width > 0 {
        if color {
            let label = line_num
                .map(|n| format!("{:>width$}", n, width = width))
                .unwrap_or_else(|| format!("{:>width$}", "#", width = width));
            write!(w, "{}{} | {}", LINE_NUM_COLOR, label, RESET_CODE).unwrap();
        } else {
            let label = line_num
                .map(|n| n.to_string())
                .unwrap_or_else(|| "#".to_string());
            write!(w, "{:>width$} | ", label, width = width).unwrap();
        }
    }

    for (idx, elem) in row.iter().enumerate() {
        if color {
            let color_code = ANSI_COLOR_CODES[idx % ANSI_COLOR_CODES.len()];
            write!(w, "{}{} ", color_code, elem).unwrap();
        } else {
            if idx > 0 {
                write!(w, " ").unwrap();
            }
            write!(w, "{}", elem).unwrap();
        }
    }

    if color {
        writeln!(w, "{}", RESET_CODE).unwrap();
    } else {
        writeln!(w).unwrap();
    }
}

struct Args {
    filename: Option<String>,
    show_line_numbers: bool,
    no_color: bool,
}

fn parse_args(iter: impl Iterator<Item = String>) -> Args {
    let mut filename = None;
    let mut show_line_numbers = false;
    let mut no_color = false;

    for arg in iter {
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
    let args = parse_args(env::args().skip(1));
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
    let stdout = io::stdout();
    let mut out = stdout.lock();

    if args.show_line_numbers {
        let records: Vec<StringRecord> = rdr.records().collect::<Result<_, _>>()?;
        let width = records.len().to_string().len().max(1);

        let header = rdr.headers()?;
        print_row(&mut out, &header, None, width, color);

        for (i, record) in records.iter().enumerate() {
            print_row(&mut out, record, Some(i + 1), width, color);
        }
    } else {
        let header = rdr.headers()?;
        print_row(&mut out, &header, None, 0, color);

        for record in rdr.records() {
            let record = record?;
            print_row(&mut out, &record, None, 0, color);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use csv::StringRecord;

    // --- parse_args ---

    #[test]
    fn parse_args_short_n() {
        let args = parse_args(["-n"].iter().map(|s| s.to_string()));
        assert!(args.show_line_numbers);
        assert!(!args.no_color);
        assert!(args.filename.is_none());
    }

    #[test]
    fn parse_args_long_line_number() {
        let args = parse_args(["--line-number"].iter().map(|s| s.to_string()));
        assert!(args.show_line_numbers);
    }

    #[test]
    fn parse_args_short_c() {
        let args = parse_args(["-C"].iter().map(|s| s.to_string()));
        assert!(args.no_color);
        assert!(!args.show_line_numbers);
        assert!(args.filename.is_none());
    }

    #[test]
    fn parse_args_long_no_color() {
        let args = parse_args(["--no-color"].iter().map(|s| s.to_string()));
        assert!(args.no_color);
    }

    #[test]
    fn parse_args_filename() {
        let args = parse_args(["file.csv"].iter().map(|s| s.to_string()));
        assert_eq!(args.filename, Some("file.csv".to_string()));
        assert!(!args.show_line_numbers);
        assert!(!args.no_color);
    }

    #[test]
    fn parse_args_all_flags_and_filename() {
        let args = parse_args(["-n", "-C", "file.csv"].iter().map(|s| s.to_string()));
        assert!(args.show_line_numbers);
        assert!(args.no_color);
        assert_eq!(args.filename, Some("file.csv".to_string()));
    }

    #[test]
    fn parse_args_filename_before_flags() {
        let args = parse_args(["file.csv", "-n", "-C"].iter().map(|s| s.to_string()));
        assert!(args.show_line_numbers);
        assert!(args.no_color);
        assert_eq!(args.filename, Some("file.csv".to_string()));
    }

    #[test]
    fn parse_args_unknown_flag_ignored() {
        let args = parse_args(["--unknown", "file.csv"].iter().map(|s| s.to_string()));
        assert_eq!(args.filename, Some("file.csv".to_string()));
    }

    #[test]
    fn parse_args_empty() {
        let args = parse_args(std::iter::empty());
        assert!(args.filename.is_none());
        assert!(!args.show_line_numbers);
        assert!(!args.no_color);
    }

    // --- print_row: no-color, no line number ---

    fn record(fields: &[&str]) -> StringRecord {
        StringRecord::from(fields.to_vec())
    }

    #[test]
    fn print_row_no_color_single_field() {
        let mut out = Vec::new();
        print_row(&mut out, &record(&["hello"]), None, 0, false);
        assert_eq!(String::from_utf8(out).unwrap(), "hello\n");
    }

    #[test]
    fn print_row_no_color_multiple_fields() {
        let mut out = Vec::new();
        print_row(&mut out, &record(&["foo", "bar", "baz"]), None, 0, false);
        assert_eq!(String::from_utf8(out).unwrap(), "foo bar baz\n");
    }

    // --- print_row: no-color, with line number ---

    #[test]
    fn print_row_no_color_header_marker() {
        let mut out = Vec::new();
        print_row(&mut out, &record(&["col1", "col2"]), None, 1, false);
        let s = String::from_utf8(out).unwrap();
        assert!(s.starts_with("# | "), "got: {:?}", s);
    }

    #[test]
    fn print_row_no_color_line_number() {
        let mut out = Vec::new();
        print_row(&mut out, &record(&["a", "b"]), Some(5), 1, false);
        let s = String::from_utf8(out).unwrap();
        assert!(s.starts_with("5 | "), "got: {:?}", s);
    }

    #[test]
    fn print_row_no_color_line_number_padding() {
        let mut out = Vec::new();
        // width=4 (for 1000-row file), row 1 should be right-padded
        print_row(&mut out, &record(&["x"]), Some(1), 4, false);
        let s = String::from_utf8(out).unwrap();
        assert!(s.starts_with("   1 | "), "got: {:?}", s);
    }

    // --- print_row: color mode ---

    #[test]
    fn print_row_color_contains_ansi_codes() {
        let mut out = Vec::new();
        print_row(&mut out, &record(&["foo", "bar"]), None, 0, true);
        let s = String::from_utf8(out).unwrap();
        assert!(s.contains("\x1b["), "expected ANSI codes");
    }

    #[test]
    fn print_row_color_ends_with_reset() {
        let mut out = Vec::new();
        print_row(&mut out, &record(&["foo"]), None, 0, true);
        let s = String::from_utf8(out).unwrap();
        assert!(s.contains(RESET_CODE));
    }

    #[test]
    fn print_row_color_contains_field_value() {
        let mut out = Vec::new();
        print_row(&mut out, &record(&["hello", "world"]), None, 0, true);
        let s = String::from_utf8(out).unwrap();
        assert!(s.contains("hello"));
        assert!(s.contains("world"));
    }

    #[test]
    fn print_row_color_cycles_after_seven_columns() {
        let fields: Vec<&str> = vec!["a", "b", "c", "d", "e", "f", "g", "h"];
        let mut out = Vec::new();
        print_row(&mut out, &record(&fields), None, 0, true);
        let s = String::from_utf8(out).unwrap();
        // color for col 0 and col 7 should both be ANSI_COLOR_CODES[0]
        let first_occurrence = s.find(ANSI_COLOR_CODES[0]).unwrap();
        let second_occurrence = s.rfind(ANSI_COLOR_CODES[0]).unwrap();
        assert_ne!(first_occurrence, second_occurrence, "color should cycle");
    }

    #[test]
    fn print_row_color_line_number_uses_dim_code() {
        let mut out = Vec::new();
        print_row(&mut out, &record(&["x"]), Some(1), 1, true);
        let s = String::from_utf8(out).unwrap();
        assert!(s.starts_with(LINE_NUM_COLOR), "got: {:?}", s);
    }
}
