use anyhow;
use assert_cmd::Command;
use common::{INPUT_FILE, PRG_NAME};
use predicates::prelude::*;

mod common;

#[test]
fn dies_when_args_are_unknown() -> anyhow::Result<()> {
    Command::cargo_bin(PRG_NAME)?
        .args(&["-b", INPUT_FILE])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "error: unexpected argument '-b' found",
        ));

    Ok(())
}

#[test]
fn dies_with_both_unique_and_duplicates_args() -> anyhow::Result<()> {
    Command::cargo_bin(PRG_NAME)?
        .args(&["-ud", INPUT_FILE])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "error: the argument '--unique' cannot be used with '--repeated'",
        ));

    Ok(())
}
