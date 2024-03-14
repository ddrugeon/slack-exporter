use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn it_should_fails_with_no_args() {
    Command::cargo_bin("slack-exporter")
        .unwrap()
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}
