use std::fs;
use std::process::Command;

use tempfile::NamedTempFile;

fn create_test_file(content: &str) -> NamedTempFile {
    let file = NamedTempFile::new().expect("failed to create temporary file");
    fs::write(file.path(), content).expect("failed to write temporary file");
    file
}

fn run_textkit(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_textkit"))
        .args(args)
        .output()
        .expect("failed to run textkit")
}

#[test]
fn help_lists_all_subcommands() {
    let output = run_textkit(&["--help"]);

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();

    for command in ["stats", "search", "filter", "replace"] {
        assert!(
            stdout.contains(command),
            "help output should contain {command}"
        );
    }
}

#[test]
fn stats_command_prints_text_statistics() {
    let file = create_test_file("Rust is fast.\n你好");

    let output = run_textkit(&["stats", file.path().to_str().unwrap()]);

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.contains("Lines: 2"));
    assert!(stdout.contains("Words: 4"));
    assert!(stdout.contains("Characters: 16"));
    assert!(stdout.contains("Bytes: 20"));
}

#[test]
fn search_command_prints_matching_lines() {
    let file = create_test_file("Rust is fast.\nPython is readable.\nI like Rust.\n");

    let output = run_textkit(&["search", file.path().to_str().unwrap(), "Rust"]);

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.contains("1: Rust is fast."));
    assert!(stdout.contains("3: I like Rust."));
    assert!(stdout.contains("Found 2 matching lines."));
}

#[test]
fn replace_command_reports_replacement_count() {
    let file = create_test_file("Rust is fast. Rust is safe.");

    let output = run_textkit(&["replace", file.path().to_str().unwrap(), "Rust", "Go"]);

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.contains("Go is fast. Go is safe."));
    assert!(stdout.contains("Replacements: 2"));
}

#[test]
fn missing_file_returns_error() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("missing.txt");

    let output = run_textkit(&["stats", path.to_str().unwrap()]);

    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("I/O error"));
}

#[test]
fn invalid_filter_mode_returns_error() {
    let file = create_test_file("Rust\nGo");

    let output = run_textkit(&[
        "filter",
        file.path().to_str().unwrap(),
        "Rust",
        "--mode",
        "unknown",
    ]);

    assert!(!output.status.success());
}

#[test]
fn missing_required_arguments_returns_error() {
    let output = run_textkit(&["search"]);

    assert!(!output.status.success());
}
