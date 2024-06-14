use anyhow;
use assert_cmd::Command;
use common::{INPUT_FILE, PRG_NAME};
use std::fs;
use tempfile::NamedTempFile;

mod common;

#[test]
fn with_no_options() -> anyhow::Result<()> {
    let res = Command::cargo_bin(PRG_NAME)?.args(&[INPUT_FILE]).output()?;
    assert!(res.status.success());
    assert_eq!(
        String::from_utf8(res.stdout)?,
        vec![
            "summer", " ", "summer", "SummeR", "came", "CAME", "Came ", "quite", "early", "quite",
            "QUITE", "quite\n",
        ]
        .join("\n")
    );

    Ok(())
}

#[test]
fn case_insensitive_duplicate_counts() -> anyhow::Result<()> {
    let res = Command::cargo_bin(PRG_NAME)?
        .args(&[INPUT_FILE, "-cid"])
        .output()?;
    assert!(res.status.success());
    assert_eq!(
        String::from_utf8(res.stdout)?,
        vec![
            format!("{:>5} summer", 2),
            format!("{:>5} came", 2),
            format!("{:>5} quite\n", 3),
        ]
        .join("\n")
    );

    Ok(())
}

#[test]
fn writes_to_file() -> anyhow::Result<()> {
    let file = NamedTempFile::new()?;
    let filepath = &file.path().to_str().unwrap();
    Command::cargo_bin(PRG_NAME)?
        .args(&[INPUT_FILE, filepath, "-cid"])
        .assert()
        .success()
        .stdout("");
    assert_eq!(
        fs::read_to_string(filepath)?,
        vec![
            format!("{:>5} summer", 2),
            format!("{:>5} came", 2),
            format!("{:>5} quite\n", 3),
        ]
        .join("\n")
    );

    Ok(())
}
