use assert_cmd::Command;
use predicates::prelude::*;

const PRG: &'static str = "findr";

#[test]
fn dies_bad_type() -> anyhow::Result<()> {
    run_bad_arg_test(
        &["--type", "x"],
        "invalid value 'x' for '--type [<TYPE>...]'",
    )
}

#[test]
fn dies_bad_name_regex() -> anyhow::Result<()> {
    run_bad_arg_test(
        &["--name", "*.csv"],
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
    run_stdout_test(
        &["tests/inputs"],
        &mut vec![
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
        ],
    )
}

#[test]
fn list_entries_in_multiple_directories() -> anyhow::Result<()> {
    run_stdout_test(
        &["tests/inputs/d", "tests/inputs/f"],
        &mut vec![
            "tests/inputs/d",
            "tests/inputs/d/b.csv",
            "tests/inputs/d/d.txt",
            "tests/inputs/d/d.tsv",
            "tests/inputs/d/e",
            "tests/inputs/d/e/e.mp3",
            "tests/inputs/f",
            "tests/inputs/f/f.txt",
        ],
    )
}

#[test]
fn list_directories() -> anyhow::Result<()> {
    run_stdout_test(
        &["tests", "--type", "d"],
        &mut vec![
            "tests",
            "tests/inputs",
            "tests/inputs/a",
            "tests/inputs/a/b",
            "tests/inputs/a/b/c",
            "tests/inputs/f",
            "tests/inputs/d",
            "tests/inputs/d/e",
        ],
    )
}

#[test]
fn list_regular_files() -> anyhow::Result<()> {
    run_stdout_test(&["tests/inputs", "--type", "f"], &mut regular_files())
}

#[cfg(not(windows))]
fn regular_files() -> Vec<&'static str> {
    vec![
        "tests/inputs/g.csv",
        "tests/inputs/a/a.txt",
        "tests/inputs/a/b/b.csv",
        "tests/inputs/a/b/c/c.mp3",
        "tests/inputs/f/f.txt",
        "tests/inputs/d/d.txt",
        "tests/inputs/d/d.tsv",
        "tests/inputs/d/e/e.mp3",
    ]
}

#[cfg(windows)]
fn regular_files() -> Vec<&'static str> {
    vec![
        "tests/inputs/g.csv",
        "tests/inputs/a/a.txt",
        "tests/inputs/a/b/b.csv",
        "tests/inputs/a/b/c/c.mp3",
        "tests/inputs/f/f.txt",
        "tests/inputs/d/d.txt",
        "tests/inputs/d/d.tsv",
        "tests/inputs/d/e/e.mp3",
        // symbolic links are regular files on windows
        "tests/inputs/d/b.csv",
    ]
}

#[test]
#[cfg(not(windows))]
// symbolic links are regular files on windows
fn list_symbolic_link() -> anyhow::Result<()> {
    run_stdout_test(
        &["tests/inputs", "--type", "l"],
        &mut vec!["tests/inputs/d/b.csv"],
    )
}

#[test]
#[cfg(not(windows))]
// symbolic links are regular files on windows
fn find_by_multiple_types() -> anyhow::Result<()> {
    run_stdout_test(
        &["tests/inputs/d", "--type", "l", "--type", "d"],
        &mut vec!["tests/inputs/d", "tests/inputs/d/b.csv", "tests/inputs/d/e"],
    )
}

#[test]
fn find_by_name() -> anyhow::Result<()> {
    run_stdout_test(
        &["tests/inputs", "--name", ".*[.]csv"],
        &mut vec![
            "tests/inputs/g.csv",
            "tests/inputs/a/b/b.csv",
            "tests/inputs/d/b.csv",
        ],
    )
}

#[test]
fn find_by_multiple_names() -> anyhow::Result<()> {
    run_stdout_test(
        &["tests/inputs", "--name", ".*[.]csv", "--name", ".*[.]txt"],
        &mut vec![
            "tests/inputs/g.csv",
            "tests/inputs/a/a.txt",
            "tests/inputs/a/b/b.csv",
            "tests/inputs/f/f.txt",
            "tests/inputs/d/b.csv",
            "tests/inputs/d/d.txt",
        ],
    )
}

#[test]
#[cfg(not(windows))]
fn find_symbolic_links_by_name() -> anyhow::Result<()> {
    run_stdout_test(
        &["tests/inputs", "--name", ".*[.]csv", "--type", "l"],
        &mut vec!["tests/inputs/d/b.csv"],
    )
}

#[test]
fn find_regular_files_by_name() -> anyhow::Result<()> {
    run_stdout_test(
        &["tests/inputs", "--name", ".*[.]csv", "--type", "f"],
        &mut vec!["tests/inputs/g.csv", "tests/inputs/a/b/b.csv"],
    )
}

#[test]
fn restrict_to_maxdepth() -> anyhow::Result<()> {
    run_stdout_test(
        &["tests/inputs", "--maxdepth", "1"],
        &mut vec![
            "tests/inputs",
            "tests/inputs/g.csv",
            "tests/inputs/a",
            "tests/inputs/f",
            "tests/inputs/d",
        ],
    )?;
    run_stdout_test(
        &["tests/inputs/a", "--maxdepth", "2"],
        &mut vec![
            "tests/inputs/a",
            "tests/inputs/a/a.txt",
            "tests/inputs/a/b",
            "tests/inputs/a/b/b.csv",
            "tests/inputs/a/b/c",
        ],
    )
}

#[test]
fn restrict_to_mindepth() -> anyhow::Result<()> {
    run_stdout_test(
        &["tests/inputs", "--mindepth", "4"],
        &mut vec!["tests/inputs/a/b/c/c.mp3"],
    )?;
    run_stdout_test(
        &["tests/inputs/a", "--mindepth", "2"],
        &mut vec![
            "tests/inputs/a/b/b.csv",
            "tests/inputs/a/b/c",
            "tests/inputs/a/b/c/c.mp3",
        ],
    )
}

#[test]
fn restrict_to_within_depth_range() -> anyhow::Result<()> {
    run_stdout_test(
        &["tests/inputs", "--mindepth", "3", "--maxdepth", "4"],
        &mut vec![
            "tests/inputs/a/b/b.csv",
            "tests/inputs/a/b/c",
            "tests/inputs/a/b/c/c.mp3",
            "tests/inputs/d/e/e.mp3",
        ],
    )?;
    run_stdout_test(
        &["tests/inputs", "--mindepth", "3", "--maxdepth", "3"],
        &mut vec![
            "tests/inputs/a/b/b.csv",
            "tests/inputs/a/b/c",
            "tests/inputs/d/e/e.mp3",
        ],
    )
}

fn run_stdout_test(args: &[&str], out: &mut [&str]) -> anyhow::Result<()> {
    let res = Command::cargo_bin(PRG)?.args(args).output()?;
    assert!(
        res.status.success(),
        "command is successful: {}",
        String::from_utf8(res.stderr)?
    );
    let actual = String::from_utf8(res.stdout)?;
    let mut actual = actual
        .split('\n')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    actual.sort();
    out.sort();
    assert_eq!(actual, out, "outputs match");
    Ok(())
}

fn run_stderr_test(args: &[&str], errs: &[&str]) -> anyhow::Result<()> {
    let res = Command::cargo_bin(PRG)?.args(args).output()?;

    assert!(res.status.success(), "command exits successfully");

    let actual_errs = String::from_utf8(res.stderr)?;
    let actual_errs = actual_errs.split('\n').collect::<Vec<_>>();

    assert!(actual_errs.len() > 0, "has at least 1 standard error");

    for e in errs {
        assert!(actual_errs.contains(e), "stderr contains: {e}");
    }

    Ok(())
}
