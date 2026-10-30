use std::process;
use std::{fs, path::Path};

use clap::Parser;
use textkit::cli::{Cli, CliFilterMode, Commands, FileArgs, FilterArgs, ReplaceArgs, SearchArgs};
use textkit::{FilterMode, TextkitError, count_text, filter_lines, replace_text, search_lines};

fn main() {
    let cli = Cli::parse();
    if let Err(error) = run(cli) {
        eprintln!("Error: {error}");
        process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), TextkitError> {
    match cli.command {
        Commands::Filter(args) => run_filter(args),
        Commands::Replace(args) => run_replace(args),
        Commands::Search(args) => run_search(args),
        Commands::Stats(args) => run_stats(args),
    }
}

fn read_file(path: &Path) -> Result<String, TextkitError> {
    Ok(fs::read_to_string(path)?)
}

fn run_stats(args: FileArgs) -> Result<(), TextkitError> {
    let content = read_file(&args.file)?;
    let stats = count_text(&content);

    println!("File: {}", args.file.display());
    println!("Lines: {}", stats.lines);
    println!("Words: {}", stats.words);
    println!("Characters: {}", stats.characters);
    println!("Bytes: {}", stats.bytes);

    Ok(())
}

fn run_search(args: SearchArgs) -> Result<(), TextkitError> {
    let content = read_file(&args.file)?;
    let results = search_lines(&content, &args.keyword);

    for (line_number, line) in &results {
        println!("{line_number}: {line}");
    }

    println!("Found {} matching lines.", results.len());

    Ok(())
}

fn run_filter(args: FilterArgs) -> Result<(), TextkitError> {
    let mode = match args.mode {
        CliFilterMode::Contains => FilterMode::Contains,
        CliFilterMode::Exclude => FilterMode::Exclude,
        CliFilterMode::StartsWith => FilterMode::StartsWith,
    };

    let content = read_file(&args.file)?;
    let results = filter_lines(&content, &args.keyword, mode);

    for (line_number, line) in &results {
        println!("{line_number}: {line}");
    }

    println!("Filtered {} lines.", results.len());

    Ok(())
}

fn run_replace(args: ReplaceArgs) -> Result<(), TextkitError> {
    let content = read_file(&args.file)?;
    let (result, count) = replace_text(&content, &args.pattern, &args.replacement);

    print!("{result}");
    println!("\nReplacements: {count}");

    Ok(())
}
