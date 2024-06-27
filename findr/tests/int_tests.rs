use assert_cmd::Command;
use predicates::prelude::*;

const PRG: &'static str = "findr";

#[test]
fn dies_bad_type() -> anyhow::Result<()> {
    run_bad_arg_test(&["-t", "x"], "invalid value 'x' for '--type [<TYPE>...]'")
}

#[test]
fn dies_bad_name_regex() -> anyhow::Result<()> {
    run_bad_arg_test(
        &["-n", "*.csv"],
        "error: invalid value '*.csv' for '--name [<NAME>...]'",
    )
}

fn run_bad_arg_test(args: &[&str], contained_err: &str) -> anyhow::Result<()> {
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .failure()
        .stderr(predicate::str::contains(contained_err));

    Ok(())
}

#[test]
fn writes_errors_to_standard_error() -> anyhow::Result<()> {
    run_stderr_test(
        &["tests/inputs", "blargh"],
        &vec![format!("{PRG}: blargh: File or directory not found").as_str()],
    )
}

#[test]
fn list_all_entries_in_directory() -> anyhow::Result<()> {
    let mut expected = vec![
        "tests/inputs",
        "tests/inputs/g.csv",
        "tests/inputs/a",
        "tests/inputs/a/a.txt",
        "tests/inputs/a/b",
        "tests/inputs/a/b/b.csv",
        "tests/inputs/a/b/c",
        "tests/inputs/a/b/c/c.mp3",
        "tests/inputs/f",
        "tests/inputs/f/f.txt",
        "tests/inputs/d",
        "tests/inputs/d/b.csv",
        "tests/inputs/d/d.txt",
        "tests/inputs/d/d.tsv",
        "tests/inputs/d/e",
        "tests/inputs/d/e/e.mp3",
    ]
    .join("\n");
    expected.push('\n');

    run_stdout_test(&["tests/inputs"], &expected)
}

#[test]
fn list_entries_in_multiple_directories() -> anyhow::Result<()> {
    let mut expected = vec![
        "tests/inputs/d",
        "tests/inputs/d/b.csv",
        "tests/inputs/d/d.txt",
        "tests/inputs/d/d.tsv",
        "tests/inputs/d/e",
        "tests/inputs/d/e/e.mp3",
        "tests/inputs/f",
        "tests/inputs/f/f.txt",
    ]
    .join("\n");
    expected.push('\n');

    run_stdout_test(&["tests/inputs/d", "tests/inputs/f"], &expected)
}

#[test]
fn list_directories() -> anyhow::Result<()> {
    let mut expected = vec![
        "tests",
        "tests/inputs",
        "tests/inputs/a",
        "tests/inputs/a/b",
        "tests/inputs/a/b/c",
        "tests/inputs/f",
        "tests/inputs/d",
        "tests/inputs/d/e",
    ]
    .join("\n");
    expected.push('\n');
    run_stdout_test(&["tests", "-t", "d"], &expected)
}

#[test]
fn list_regular_files() -> anyhow::Result<()> {
    let mut expected = vec![
        "tests/inputs/g.csv",
        "tests/inputs/a/a.txt",
        "tests/inputs/a/b/b.csv",
        "tests/inputs/a/b/c/c.mp3",
        "tests/inputs/f/f.txt",
        "tests/inputs/d/d.txt",
        "tests/inputs/d/d.tsv",
        "tests/inputs/d/e/e.mp3",
    ]
    .join("\n");
    expected.push('\n');
    run_stdout_test(&["tests/inputs", "-t", "f"], &expected)
}

#[test]
fn list_symbolic_link() -> anyhow::Result<()> {
    let expected = "tests/inputs/d/b.csv\n";
    run_stdout_test(&["tests/inputs", "-t", "l"], &expected)
}

#[test]
fn find_by_multiple_types() -> anyhow::Result<()> {
    let mut expected =
        vec!["tests/inputs/d", "tests/inputs/d/b.csv", "tests/inputs/d/e"].join("\n");
    expected.push('\n');
    run_stdout_test(&["tests/inputs/d", "-t", "l", "-t", "d"], &expected)
}

#[test]
fn find_by_name() -> anyhow::Result<()> {
    let mut expected = vec![
        "tests/inputs/g.csv",
        "tests/inputs/a/b/b.csv",
        "tests/inputs/d/b.csv",
    ]
    .join("\n");
    expected.push('\n');
    run_stdout_test(&["tests/inputs", "-n", ".*[.]csv"], &expected)
}

#[test]
fn find_by_multiple_names() -> anyhow::Result<()> {
    let mut expected = vec![
        "tests/inputs/g.csv",
        "tests/inputs/a/a.txt",
        "tests/inputs/a/b/b.csv",
        "tests/inputs/f/f.txt",
        "tests/inputs/d/b.csv",
        "tests/inputs/d/d.txt",
    ]
    .join("\n");
    expected.push('\n');
    run_stdout_test(
        &["tests/inputs", "-n", ".*[.]csv", "-n", ".*[.]txt"],
        &expected,
    )
}

#[test]
fn find_by_name_and_type() -> anyhow::Result<()> {
    run_stdout_test(
        &["tests/inputs", "-n", ".*[.]csv", "-t", "l"],
        "tests/inputs/d/b.csv\n",
    )?;
    run_stdout_test(
        &["tests/inputs", "-n", ".*[.]csv", "-t", "f"],
        &vec!["tests/inputs/g.csv", "tests/inputs/a/b/b.csv\n"].join("\n"),
    )
}

fn run_stdout_test(args: &[&str], out: &str) -> anyhow::Result<()> {
    let res = Command::cargo_bin(PRG)?.args(args).output()?;
    let actual = String::from_utf8(res.stdout)?;
    assert!(
        res.status.success(),
        "command is successful: {}",
        String::from_utf8(res.stderr)?
    );
    assert_eq!(actual, out, "outputs match");
    Ok(())
}

fn run_stderr_test(args: &[&str], errs: &[&str]) -> anyhow::Result<()> {
    let res = Command::cargo_bin(PRG)?.args(args).output()?;

    assert!(res.status.success(), "command exits successfully");

    let actual_errs = String::from_utf8(res.stderr)?;
    let actual_errs = actual_errs.split('\n').collect::<Vec<_>>();

    assert!(actual_errs.len() > 0, "has at least 1standard error");

    for e in errs {
        assert!(actual_errs.contains(e), "stderr contains: {e}");
    }

    Ok(())
}
