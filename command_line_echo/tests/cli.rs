use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("command_line_echo").unwrap();

    cmd.arg("hello").assert().success();
}

#[test]
fn dies_no_args() {
    let mut cmd = Command::cargo_bin("command_line_echo").unwrap();

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
}