// use std::process::Command;
use assert_cmd::Command;

#[test]
fn works() {
    // let mut cmd = Command::cargo_bin("hello").unwrap();
    // cmd.assert().success();
    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.assert().success().stdout("Hello, world!!!\n");
    // let mut cmd = Command::new("hello");
    // let res = cmd.output();
    // assert!(res.is_ok());
    // assert!(true);
    // assert!(false);
}

#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn false_ok() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}
