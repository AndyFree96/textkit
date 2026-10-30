use std::{fs, process::Command};

fn create_test_file(name: &str, content: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!("textkit-{name}-{}.txt", std::process::id()));

    fs::write(&path, content).expect("failed to create test file");
    path
}

#[test]
fn search_command_prints_matching_lines() {
    let path = create_test_file(
        "search",
        "Rust is fast.\nPython is readable.\nI like Rust.\n",
    );

    let output = Command::new(env!("CARGO_BIN_EXE_textkit"))
        .args(["search"])
        .arg(&path)
        .arg("Rust")
        .output()
        .expect("failed to run textkit");

    let _ = fs::remove_file(&path);

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.contains("1: Rust is fast."));
    assert!(stdout.contains("3: I like Rust."));
    assert!(stdout.contains("Found 2 matching lines."));
}

#[test]
fn missing_file_returns_error() {
    let path = std::env::temp_dir().join(format!("textkit-missing-{}.txt", std::process::id()));

    let _ = fs::remove_file(&path);

    let output = Command::new(env!("CARGO_BIN_EXE_textkit"))
        .arg("stats")
        .arg(&path)
        .output()
        .expect("failed to run textkit");

    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("I/O error"));
}

#[test]
fn help_lists_all_subcommands() {
    let output = Command::new(env!("CARGO_BIN_EXE_textkit"))
        .arg("--help")
        .output()
        .expect("failed to run textkit");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.contains("stats"));
    assert!(stdout.contains("search"));
    assert!(stdout.contains("filter"));
    assert!(stdout.contains("replace"));
}
