use std::process;
use std::{env, fs};

use textkit::{FilterMode, TextkitError, count_text, filter_lines, replace_text, search_lines};

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {error}");
        process::exit(1);
    }
}

fn run() -> Result<(), TextkitError> {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(String::as_str) {
        Some("search") => run_search(&args),
        Some("filter") => run_filter(&args),
        Some("replace") => run_replace(&args),
        Some(_) if args.len() == 2 => run_stats(&args[1]),
        _ => Err(TextkitError::InvalidArguments(usage())),
    }
}

fn read_file(path: &str) -> Result<String, TextkitError> {
    Ok(fs::read_to_string(path)?)
}

fn run_stats(path: &str) -> Result<(), TextkitError> {
    let content = read_file(path)?;
    let stats = count_text(&content);

    println!("File: {path}");
    println!("Lines: {}", stats.lines);
    println!("Words: {}", stats.words);
    println!("Characters: {}", stats.characters);
    println!("Bytes: {}", stats.bytes);

    Ok(())
}

fn run_search(args: &[String]) -> Result<(), TextkitError> {
    if args.len() != 4 {
        return Err(TextkitError::InvalidArguments(usage()));
    }

    let content = read_file(&args[2])?;
    let results = search_lines(&content, &args[3]);

    for (line_number, line) in &results {
        println!("{line_number}: {line}");
    }

    println!("Found {} matching lines.", results.len());

    Ok(())
}

fn run_filter(args: &[String]) -> Result<(), TextkitError> {
    if args.len() != 4 && args.len() != 5 {
        return Err(TextkitError::InvalidArguments(usage()));
    }

    let mode = match args.get(4).map(String::as_str) {
        None => FilterMode::Contains,
        Some("--exclude") => FilterMode::Exclude,
        Some("--starts-with") => FilterMode::StartsWith,
        Some(_) => return Err(TextkitError::InvalidArguments(usage())),
    };

    let content = read_file(&args[2])?;
    let results = filter_lines(&content, &args[3], mode);

    for (line_number, line) in &results {
        println!("{line_number}: {line}");
    }

    println!("Filtered {} lines.", results.len());

    Ok(())
}

fn run_replace(args: &[String]) -> Result<(), TextkitError> {
    if args.len() != 5 {
        return Err(TextkitError::InvalidArguments(usage()));
    }

    let content = read_file(&args[2])?;
    let (result, count) = replace_text(&content, &args[3], &args[4]);

    print!("{result}");
    println!("\nReplacements: {count}");

    Ok(())
}

fn usage() -> String {
    [
        "Usage:",
        "  textkit <file>",
        "  textkit search <file> <keyword>",
        "  textkit filter <file> <keyword> [--exclude|--starts-with]",
        "  textkit replace <file> <pattern> <replacement>",
    ]
    .join("\n")
}
