use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_runs() {
    let mut cmd = Command::cargo_bin("bookr").unwrap();
    cmd.arg("--help").assert().success();
}

#[test]
fn test_fails() {
    let mut cmd = Command::cargo_bin("bookr").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage: bookr"));
}

fn test_add_contact_to_phonebook() {
    let mut cmd = Command::cargo_bin("bookr").unwrap();
    cmd.arg("add")
        .arg("Roger Waters")
        .arg("905-555-9999")
        .assert()
        .success();
    cmd.arg("lookup")
        .arg("--name")
        .arg("Roger Waters")
        .assert()
        .success()
        .stdout(predicate::str::contains("905-555-9999"));
}

#[test]
fn test_edit_contact_name_in_phonebook() {
    let mut cmd = Command::cargo_bin("bookr").unwrap();
    cmd.arg("edit")
        .arg("--name")
        .arg("Roger Waters")
        .arg("--new-name")
        .arg("David Gilmour")
        .assert()
        .success();
}

#[test]
fn test_edit_contact_number_in_phonebook() {
    let mut cmd = Command::cargo_bin("bookr").unwrap();
    cmd.arg("edit")
        .arg("--name")
        .arg("Roger Waters")
        .arg("--number")
        .arg("416-555-9999")
        .assert()
        .success();
}

