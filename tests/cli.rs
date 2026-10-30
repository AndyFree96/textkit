use assert_cmd::Command;
use std::fs;
use tempfile::NamedTempFile;

fn create_test_file(content: &str) -> NamedTempFile {
    let file = NamedTempFile::new().expect("failed to create temporary file");
    fs::write(file.path(), content).expect("failed to write temporary file");
    file
}

#[test]
fn help_lists_all_subcommands() {
    let mut cmd = Command::cargo_bin("textkit").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicates::str::contains("stats"))
        .stdout(predicates::str::contains("search"))
        .stdout(predicates::str::contains("filter"))
        .stdout(predicates::str::contains("replace"));
}

#[test]
fn stats_command_prints_text_statistics() {
    let file = create_test_file("Rust is fast.\n你好");

    let mut cmd = Command::cargo_bin("textkit").unwrap();
    cmd.args(["stats", file.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicates::str::contains("Lines: 2"))
        .stdout(predicates::str::contains("Words: 4"))
        .stdout(predicates::str::contains("Characters: 16"))
        .stdout(predicates::str::contains("Bytes: 20"));
}

#[test]
fn search_command_prints_matching_lines() {
    let file = create_test_file("Rust is fast.\nPython is readable.\nI like Rust.\n");

    let mut cmd = Command::cargo_bin("textkit").unwrap();
    cmd.args(["search", file.path().to_str().unwrap(), "Rust"])
    .assert()
    .success()
    .stdout(predicates::str::contains("1: Rust is fast."))
    .stdout(predicates::str::contains("3: I like Rust."))
    .stdout(predicates::str::contains("Found 2 matching lines."));
}

#[test]
fn replace_command_reports_replacement_count() {
    let file = create_test_file("Rust is fast. Rust is safe.");
    let mut cmd = Command::cargo_bin("textkit").unwrap();
    cmd.args(["replace", file.path().to_str().unwrap(), "Rust", "Go"])
    .assert()
    .success()
    .stdout(predicates::str::contains("Go is fast. Go is safe."))
    .stdout(predicates::str::contains("Replacements: 2"));
}

#[test]
fn missing_file_returns_error() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("missing.txt");

    let mut cmd = Command::cargo_bin("textkit").unwrap();
    cmd.args(["stats", path.to_str().unwrap()])
    .assert()
    .failure()
    .stderr(predicates::str::contains("I/O error"));
}

#[test]
fn invalid_filter_mode_returns_error() {
    let file = create_test_file("Rust\nGo");

    let mut cmd = Command::cargo_bin("textkit").unwrap();
    cmd.args([
        "filter",
        file.path().to_str().unwrap(),
        "Rust",
        "--mode",
        "unknown",
    ])
    .assert()
    .failure()
    .stderr(predicates::str::contains("invalid value"));
}

#[test]
fn missing_required_arguments_returns_error() {
    let mut cmd = Command::cargo_bin("textkit").unwrap();
    cmd.arg("search").assert().failure();
}
