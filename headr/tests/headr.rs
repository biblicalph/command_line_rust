use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use pretty_assertions::assert_eq;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use ctor::{ctor,dtor};

const PROGRAM_BIN: &str = "headr";
const EMPTY_FILE: &str = "tests/inputs/empty.txt";
const ONE_LINE_FILE: &str = "tests/inputs/oneline.txt";
const FOURTEEN_FILE: &str = "tests/inputs/fourteen.txt";
const MISSING_FILE: &str = "tests/inputs/fifteen.txt";
const PERM_ERROR_FILE: &str = "tests/inputs/perm_error.txt";
const MULTIBYTE_FILE: &str = "tests/inputs/multibyte.txt";

#[test]
fn dies_with_both_lines_and_bytes_arg() -> Result<()> {
    run_dies_test(
        &["-c", "10", "-n", "20"],
        "the argument '--bytes <BYTES>' cannot be used with '--lines <LINES>'",
    )
}

#[test]
fn dies_bad_bytes_or_lines() -> Result<()> {
    run_dies_test(
        &["-c", "unknown"],
        "invalid value 'unknown' for '--bytes <BYTES>'",
    )?;
    run_dies_test(
        &["-n", "unknown"],
        "invalid value 'unknown' for '--lines <LINES>'",
    )?;
    run_dies_test(&["-n", "0"], "invalid value '0' for '--lines <LINES>'")?;
    run_dies_test(&["-c", "0"], "invalid value '0' for '--bytes <BYTES>'")
}

#[test]
fn dies_file_not_found() -> Result<()> {
    run_dies_test(
        &["-c", "10", MISSING_FILE],
        &format!("{PROGRAM_BIN}: {MISSING_FILE}: File or directory not found"),
    )?;
    run_dies_test(
        &["-n", "10", MISSING_FILE],
        &format!("{PROGRAM_BIN}: {MISSING_FILE}: File or directory not found"),
    )
}

#[test]
fn dies_file_permission_error() -> Result<()> {
    run_dies_test(
        &["-c", "10", PERM_ERROR_FILE],
        &format!("{PROGRAM_BIN}: {PERM_ERROR_FILE}: Permission denied"),
    )?;
    run_dies_test(
        &["-n", "10", PERM_ERROR_FILE],
        &format!("{PROGRAM_BIN}: {PERM_ERROR_FILE}: Permission denied"),
    )
}

#[test]
fn reads_lines() -> Result<()> {
    // reads first n lines
    read_lines_or_bytes(
        &[
            "-n",
            "2",
            EMPTY_FILE,
            ONE_LINE_FILE,
            "blargh",
            FOURTEEN_FILE,
            PERM_ERROR_FILE,
        ],
        None,
        &vec![
            format_output_header(EMPTY_FILE),
            "".to_string(),
            format_output_header(ONE_LINE_FILE),
            "file with one line of text".to_string(),
            "".to_string(),
            format_output_header(FOURTEEN_FILE),
            "one".to_string(),
            "two".to_string(),
        ]
        .join("\n"),
        &vec![
            format!("{PROGRAM_BIN}: blargh: File or directory not found"),
            format!("{PROGRAM_BIN}: {PERM_ERROR_FILE}: Permission denied"),
        ]
        .join("\n"),
    )?;
    // reads all but last n lines
    read_lines_or_bytes(
        &[
            "-n",
            "-5",
            EMPTY_FILE,
            ONE_LINE_FILE,
            FOURTEEN_FILE
        ],
        None,
        &vec![
            format_output_header(EMPTY_FILE),
            "".to_string(),
            format_output_header(ONE_LINE_FILE),
            "".to_string(),
            format_output_header(FOURTEEN_FILE),
            "one".to_string(),
            "two".to_string(),
            "three".to_string(),
            "four".to_string(),
            "five".to_string(),
            "six".to_string(),
            "seven".to_string(),
            "eight".to_string(),
            "nine".to_string(),
        ]
        .join("\n"),
        "",
    )?;
    // reads with arg suffix
    read_lines_or_bytes(
        &["-n", "2K", FOURTEEN_FILE],
        None,
        &vec![
            format_output_header(FOURTEEN_FILE),
            "one".to_string(),
            "two".to_string(),
            "three".to_string(),
            "four".to_string(),
            "five".to_string(),
            "six".to_string(),
            "seven".to_string(),
            "eight".to_string(),
            "nine".to_string(),
            "ten".to_string(),
            "eleven".to_string(),
            "twelve".to_string(),
            "thirteen".to_string(),
            "fourteen".to_string(),
        ]
        .join("\n"),
        "",
    )?;
    // reads from stdin
    read_lines_or_bytes(
        &["-n", "2"],
        Some(&fs::read_to_string(FOURTEEN_FILE)?),
        &vec![
            format_output_header("-"),
            "one".to_string(),
            "two".to_string(),
        ]
        .join("\n"),
        "",
    )
}

#[test]
fn reads_bytes() -> Result<()> {
    // read first c bytes
    read_lines_or_bytes(
        &[
            "-c",
            "20",
            EMPTY_FILE,
            ONE_LINE_FILE,
            "blargh",
            MULTIBYTE_FILE,
            PERM_ERROR_FILE,
        ],
        None,
        &vec![
            format_output_header(EMPTY_FILE),
            "".to_string(),
            format_output_header(ONE_LINE_FILE),
            "file with one line o".to_string(),
            "".to_string(),
            format_output_header(MULTIBYTE_FILE),
            "Great to have a smil".to_string(),
        ]
        .join("\n"),
        &vec![
            format!("{PROGRAM_BIN}: blargh: File or directory not found"),
            format!("{PROGRAM_BIN}: {PERM_ERROR_FILE}: Permission denied"),
        ]
        .join("\n"),
    )?;
    // read all but last c bytes
    read_lines_or_bytes(
        &[
            "-c",
            "-20",
            EMPTY_FILE,
            ONE_LINE_FILE,
            MULTIBYTE_FILE,
        ],
        None,
        &vec![
            format_output_header(EMPTY_FILE),
            "".to_string(),
            format_output_header(ONE_LINE_FILE),
            "file wi".to_string(),
            "".to_string(),
            format_output_header(MULTIBYTE_FILE),
            "Great to have a smile 😊. Start the day bright".to_string(),
            r#"ありがとう (Arigatou - "Thank you" in Japanese)"#.to_string(),
            "It".to_string(),
        ]
        .join("\n"),
        "",
    )?;
    // reading multibyte characters partially
    read_lines_or_bytes(
        &[
            "-c",
            "25",
            MULTIBYTE_FILE,
        ],
        None,
        &vec![
            format_output_header(MULTIBYTE_FILE),
            "Great to have a smile �".to_string(),
        ].join("\n"),
        "",
    )?;
    // reading entire multibyte file
    read_lines_or_bytes(
        &[
            "-c",
            "2K",
            MULTIBYTE_FILE,
        ],
        None,
        &vec![
            format_output_header(MULTIBYTE_FILE),
            fs::read_to_string(MULTIBYTE_FILE)?.trim_end().to_string(),
        ].join("\n"),
        "",
    )?;

    Ok(())
}

fn run_dies_test(args: &[&str], expected_err: &str) -> Result<()> {
    Command::cargo_bin(PROGRAM_BIN)?
        .args(args)
        .assert()
        .stderr(predicate::str::contains(expected_err));

    Ok(())
}

fn read_lines_or_bytes(
    args: &[&str],
    std_input: Option<&str>,
    expected_output: &str,
    expected_err: &str,
) -> Result<()> {
    let res = match std_input {
        None => Command::cargo_bin(PROGRAM_BIN)?.args(args).output()?,
        Some(input) => Command::cargo_bin(PROGRAM_BIN)?
            .args(args)
            .write_stdin(input)
            .output()?,
    };

    assert!(res.status.success(), "read lines succeeds");
    assert_eq!(
        String::from_utf8(res.stdout)?.trim_end(),
        expected_output,
        "stdout contains expected output"
    );
    assert_eq!(
        String::from_utf8(res.stderr)?.trim_end(),
        expected_err,
        "stderr contains expected output"
    );

    Ok(())
}

fn format_output_header(filename: &str) -> String {
    format!("==> {} <==", filename)
}

/// runs before all tests are executed
#[ctor]
fn global_setup() {
    let mut perms = fs::metadata(PERM_ERROR_FILE).expect("perm error file must exist").permissions();
    perms.set_mode(0o000);
    fs::set_permissions(PERM_ERROR_FILE, perms).expect("failed to set permissions");
}

/// runs after all tests are executed
#[dtor]
fn global_teardown() {
    let mut perms = fs::metadata(PERM_ERROR_FILE).expect("perm error file must exist").permissions();
    perms.set_mode(0o644);
    fs::set_permissions(PERM_ERROR_FILE, perms).expect("failed to restore permissions");
}