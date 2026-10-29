use std::{env, fs};

use textkit::search_lines;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 && args[1] == "search" {
        run_search(&args);
    } else if  args.len() == 2 {
        run_stats(&args[1]);
    } else {
        print_usage();
        std::process::exit(1);
    }
}

fn run_search(args: &[String]) {
    if args.len() != 4 {
        print_usage();
        std::process::exit(1);
    }

    let file_path = &args[2];
    let keyword = &args[3];


    let content = read_to_string(file_path);

    let results = search_lines(&content, keyword);

    for (line_number, line) in &results {
        println!("{line_number}: {line}");
    }

    println!("Found {} matching lines.", results.len());
}


fn read_to_string(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap_or_else(|error| {
        eprintln!("Failed to read file: {error}");
        std::process::exit(1);
    })
}

fn run_stats(file_path: &str) {
    let content = read_to_string(file_path);

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
    eprintln!("    textkit search <file> <keyword>")
}