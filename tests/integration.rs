use std::path::PathBuf;
use std::process::Command;

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_clr"))
}

fn test_file(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test")
        .join(name)
}

// --- helpers ---

fn stdout(cmd: &mut Command) -> String {
    let out = cmd.output().expect("failed to run clr");
    String::from_utf8_lossy(&out.stdout).into_owned()
}

fn exit_ok(cmd: &mut Command) -> bool {
    cmd.output().expect("failed to run clr").status.success()
}

// --- argument parsing ---

#[test]
fn unknown_flag_is_ignored() {
    assert!(exit_ok(bin().arg("--unknown-flag").arg(test_file("sample.csv"))));
}

#[test]
fn flag_order_does_not_matter() {
    let a = stdout(bin().args(["-n", "-C"]).arg(test_file("sample.csv")));
    let b = stdout(bin().args(["-C", "-n"]).arg(test_file("sample.csv")));
    assert_eq!(a, b);
}

#[test]
fn long_form_flags_work() {
    let short = stdout(bin().args(["-n", "-C"]).arg(test_file("sample.csv")));
    let long = stdout(
        bin()
            .args(["--line-number", "--no-color"])
            .arg(test_file("sample.csv")),
    );
    assert_eq!(short, long);
}

#[test]
fn filename_after_flags() {
    assert!(exit_ok(bin().arg("-C").arg(test_file("sample.csv"))));
}

#[test]
fn filename_before_flags() {
    assert!(exit_ok(bin().arg(test_file("sample.csv")).arg("-C")));
}

// --- error handling ---

#[test]
fn nonexistent_file_exits_with_error() {
    let out = bin()
        .arg("/nonexistent/path/file.csv")
        .output()
        .expect("failed to run clr");
    assert!(!out.status.success());
}

// --- color mode ---

#[test]
fn default_mode_contains_ansi_codes() {
    let out = stdout(bin().arg(test_file("sample.csv")));
    assert!(out.contains("\x1b["), "expected ANSI codes in output");
}

#[test]
fn no_color_mode_has_no_ansi_codes() {
    let out = stdout(bin().arg("-C").arg(test_file("sample.csv")));
    assert!(!out.contains("\x1b["), "expected no ANSI codes in output");
}

#[test]
fn no_color_mode_contains_field_values() {
    let out = stdout(bin().arg("-C").arg(test_file("sample.csv")));
    assert!(out.contains("Yamada Taro"));
    assert!(out.contains("John Smith"));
    assert!(out.contains("Chang Lee"));
}

// --- line number mode ---

#[test]
fn line_number_mode_shows_header_marker() {
    let out = stdout(bin().arg("-n").arg(test_file("sample.csv")));
    assert!(out.contains("# |"), "expected '# |' for header row");
}

#[test]
fn line_number_mode_shows_row_numbers() {
    let out = stdout(bin().arg("-n").arg(test_file("sample.csv")));
    assert!(out.contains("1 |"));
    assert!(out.contains("2 |"));
    assert!(out.contains("3 |"));
}

#[test]
fn no_line_number_mode_has_no_pipe_separator() {
    let out = stdout(bin().arg("-C").arg(test_file("sample.csv")));
    assert!(!out.contains("| "), "expected no '| ' separator without -n");
}

// --- line number width ---

#[test]
fn line_number_width_adapts_to_row_count() {
    // sample_large.csv has 1000 rows → width should be 4
    let out = stdout(bin().args(["-n", "-C"]).arg(test_file("sample_large.csv")));
    // single-digit rows should be right-padded: "   1 |"
    assert!(out.contains("   1 |"), "expected 4-wide padding for row 1 in 1000-row file");
    // 4-digit row should not be padded: "1000 |"
    assert!(out.contains("1000 |"));
}

// --- encoding ---

#[test]
fn utf8_file_exits_successfully() {
    assert!(exit_ok(bin().arg(test_file("sample.csv"))));
}

#[test]
fn shift_jis_file_exits_successfully() {
    assert!(exit_ok(bin().arg(test_file("sample_shift_jis.csv"))));
}

// --- stdin ---

#[test]
fn reads_from_stdin_when_no_filename_given() {
    use std::io::Write;
    use std::process::Stdio;

    let mut child = bin()
        .arg("-C")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn clr");

    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"col1,col2\nfoo,bar\n")
        .unwrap();

    let out = child.wait_with_output().expect("failed to wait");
    assert!(out.status.success());
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(text.contains("foo"));
    assert!(text.contains("bar"));
}
