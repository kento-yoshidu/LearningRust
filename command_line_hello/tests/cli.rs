use assert_cmd::Command;

#[test]
fn runs() {
    // command_line_helloというバイナリが見つかるか
    let mut cmd = Command::cargo_bin("command_line_hello").unwrap();
    cmd.assert().success().stdout("Hello, World\n");
}

#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}


#[test]
fn false_not_ok() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}
