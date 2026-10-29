use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: textkit <file>");
        std::process::exit(1);
    }

    let file_path = &args[1];

    match fs::read_to_string(file_path) {
        Ok(content) => {
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
        Err(error) => {
            eprintln!("Failed to read file: {error}");
            std::process::exit(1);
        }
    }

}