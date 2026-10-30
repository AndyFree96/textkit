use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "textkit",
    version,
    about = "A lightweight text processing toolkit"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Count lines, words, characters, and bytes.
    Stats(FileArgs),
    /// Search for lines containing a keyword.
    Search(SearchArgs),
    /// Filter lines by a keyword.
    Filter(FilterArgs),
    /// Replace all occurrences of a pattern.
    Replace(ReplaceArgs),
}

#[derive(Debug, Args)]
pub struct FileArgs {
    pub file: PathBuf,
}

#[derive(Debug, Args)]
pub struct SearchArgs {
    pub file: PathBuf,
    pub keyword: String,
}

#[derive(Debug, Args)]
pub struct FilterArgs {
    pub file: PathBuf,
    pub keyword: String,
    #[arg(long, value_enum, default_value_t = CliFilterMode::Contains)]
    pub mode: CliFilterMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum CliFilterMode {
    Contains,
    Exclude,
    StartsWith,
}

#[derive(Debug, Args)]
pub struct ReplaceArgs {
    pub file: PathBuf,
    pub pattern: String,
    pub replacement: String,
}
