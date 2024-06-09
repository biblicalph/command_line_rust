use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use pretty_assertions::assert_eq;
use std::fs;

const EMPTY_FILE: &str = "tests/inputs/empty.txt";
const FOX_FILE: &str = "tests/inputs/fox.txt";
const ATLAMAL_FILE: &str = "tests/inputs/atlamal.txt";

#[test]
fn prints_usage() -> Result<()> {
    Command::cargo_bin("wcr")?
        .args(&["-h"])
        .assert()
        .stdout(predicate::str::contains("Usage: wcr [OPTIONS] [FILES]..."));
    Ok(())
}

#[test]
fn dies_when_using_both_bytes_and_chars() -> Result<()> {
    Command::cargo_bin("wcr")?
        .args(&["-cm"])
        .assert()
        .stderr(predicate::str::contains(
            "the argument '--bytes' cannot be used with '--chars'",
        ));
    Ok(())
}

#[test]
fn displays_outputs() -> Result<()> {
    // with no options specified
    run_display_test(
        &vec![ATLAMAL_FILE, EMPTY_FILE, "blargh", FOX_FILE],
        None,
        vec![
            format!("{}{:5}{:5} {}", 4, 29, 177, ATLAMAL_FILE).as_str(),
            format!("{}{:5}{:5} {}", 0, 0, 0, EMPTY_FILE).as_str(),
            format!("{}{:5}{:5} {}", 1, 9, 48, FOX_FILE).as_str(),
            format!("{}{:5}{:5} total", 5, 38, 225).as_str(),
        ]
        .as_slice(),
        &vec!["wcr: blargh: No such file or directory (os error 2)"],
    )?;
    // with line and word options for multiple files
    run_display_test(
        &vec!["-lw", ATLAMAL_FILE, FOX_FILE],
        None,
        vec![
            format!("{}{:5} {}", 4, 29, ATLAMAL_FILE).as_str(),
            format!("{}{:5} {}", 1, 9, FOX_FILE).as_str(),
            format!("{}{:5} total", 5, 38).as_str(),
        ]
        .as_slice(),
        &vec![""],
    )?;
    // with char only option for single file
    run_display_test(
        &vec!["-c", FOX_FILE],
        None,
        vec![format!("{} {}", 48, FOX_FILE).as_str()].as_slice(),
        &vec![""],
    )?;
    // from stdin
    run_display_test(
        &vec![],
        Some(&format!(
            "{}{}",
            fs::read_to_string(ATLAMAL_FILE)?,
            fs::read_to_string(EMPTY_FILE)?
        )),
        vec![format!("{}{:5}{:5} -", 4, 29, 177).as_str()].as_slice(),
        &vec![""],
    )?;
    // from stdin with options
    run_display_test(
        &vec!["-mw"],
        Some(&format!(
            "{}{}",
            fs::read_to_string(ATLAMAL_FILE)?,
            fs::read_to_string(EMPTY_FILE)?
        )),
        vec![format!("{}{:5} -", 29, 159).as_str()].as_slice(),
        &vec![""],
    )
}

fn run_display_test(
    args: &[&str],
    stdin: Option<&str>,
    outputs: &[&str],
    errors: &[&str],
) -> Result<()> {
    let res = if stdin.is_some() {
        Command::cargo_bin("wcr")?
            .args(args)
            .write_stdin(stdin.unwrap())
            .output()?
    } else {
        Command::cargo_bin("wcr")?.args(args).output()?
    };
    let actual_output = String::from_utf8_lossy(&res.stdout);
    let actual_output = actual_output.trim_end().split("\n").collect::<Vec<_>>();
    let actual_errors = String::from_utf8_lossy(&res.stderr);
    let actual_errors = actual_errors.trim_end().split("\n").collect::<Vec<_>>();

    assert!(res.status.success(), "command exits successfully");
    assert_outputs_match(&actual_output, outputs, "stdout");
    assert_outputs_match(&actual_errors, errors, "stderr");

    Ok(())
}

fn assert_outputs_match(actual: &[&str], expected: &[&str], stream_name: &str) {
    assert_eq!(
        actual.len(),
        expected.len(),
        "{} contains expected number of entries",
        stream_name
    );
    for (i, item) in expected.into_iter().enumerate() {
        assert_eq!(
            &actual.get(i).unwrap().trim(),
            &item.trim(),
            "entry {} of expected matches actual output of {}",
            i,
            stream_name
        );
    }
}
