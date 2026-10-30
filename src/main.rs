use std::{env, fs};

use textkit::{FilterMode, filter_lines, search_lines, replace_text};

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("search") => run_search(&args),
        Some("filter") => run_filter(&args),
        Some("replace") => run_replace(&args),
        Some(_) if args.len() == 2 => run_stats(&args[1]),
        _ => {
            print_usage();
            std::process::exit(1);
        }
    }
}

fn run_search(args: &[String]) {
    if args.len() != 4 {
        print_usage();
        std::process::exit(1);
    }

    let file_path = &args[2];
    let keyword = &args[3];


    let content = read_file(file_path);

    let results = search_lines(&content, keyword);

    for (line_number, line) in &results {
        println!("{line_number}: {line}");
    }

    println!("Found {} matching lines.", results.len());
}


fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap_or_else(|error| {
        eprintln!("Failed to read file: {error}");
        std::process::exit(1);
    })
}

fn run_stats(file_path: &str) {
    let content = read_file(file_path);

    let line_count = content.lines().count();
    let word_count = content.split_whitespace().count();
    let char_count = content.chars().count();
    let byte_count = content.len();

    println!("File: {file_path}");
    println!("Lines: {line_count}");
    println!("Words: {word_count}");
    println!("Chars: {char_count}");
    println!("Bytes: {byte_count}");
}

fn print_usage() {
    eprintln!("Usage:");
    eprintln!("    textkit <file>");
    eprintln!("    textkit search <file> <keyword>");
    eprintln!("    textkit fliter <file> <keyword> [--exclude|--starts-with]");
    eprintln!("    textkit replace <file> <pattern> <replacement>");
}

fn run_filter(args: &[String]) {
    if args.len() != 4 && args.len() != 5 {
        print_usage();
        std::process::exit(1);
    }

    let file_path = &args[2];
    let keyword = &args[3];

    let mode = match args.get(4).map(String::as_str) {
        None => FilterMode::Contains,
        Some("--exclude") => FilterMode::Exclude,
        Some("--starts-with") => FilterMode::StartsWith,
        Some(_) => {
            print_usage();
            std::process::exit(1);
        }
    };

    let content = read_file(file_path);
    let results = filter_lines(&content, keyword, mode);

    for (line_number, line) in &results {
        println!("{line_number}: {line}");
    }

    println!("Filtered {} lines.", results.len());
}

fn run_replace(args: &[String]) {
    if args.len() != 5 {
        print_usage();
        std::process::exit(1);
    }

    let file_path = &args[2];
    let pattern = &args[3];
    let replacement = &args[4];

    let content = read_file(file_path);
    let (result, count) = replace_text(&content, pattern, replacement);
    print!("{result}");
    println!("\nReplacements: {count}");
}