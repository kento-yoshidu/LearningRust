use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn sample() {
    let mut cmd = Command::cargo_bin("command_line_echo").unwrap();

    cmd.args(vec!["Hello there", "-n"]).assert().success().stdout("Hello there");
}

#[test]
fn sample2() {
    let mut cmd = Command::cargo_bin("command_line_echo").unwrap();

    cmd.args(vec!["Hello there"]).assert().success().stdout("Hello there\n");
}

#[test]
fn dies_no_args() -> TestResult {
    Command::cargo_bin("command_line_echo")?
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));

    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;

    Command::cargo_bin("command_line_echo")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn hello1() -> TestResult {
    run(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> TestResult {
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_no_newline() -> TestResult {
    run(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_no_newline() -> TestResult {
    run(&["-n", "Hello there"], "tests/expected/hello2.n.txt")
}