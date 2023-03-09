use super::Grep;
use clap::Parser;
use regex::Regex;
use std::{io, path::Path, process::ExitCode};
use string_utils;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Suppress all normal output
    #[arg(long, short, visible_alias = "silent")]
    quiet: bool,

    /// Search for this pattern
    #[arg()]
    pattern: String,

    /// Search in these files
    #[arg()]
    files: Vec<String>,
}

enum CliResult {
    Selected,
    NoneSelected,
    ErrorOccurred,
}

impl CliResult {
    fn to_exit_code(&self) -> ExitCode {
        ExitCode::from(match self {
            CliResult::Selected => 0,
            CliResult::NoneSelected => 1,
            CliResult::ErrorOccurred => 2,
        })
    }
}

pub fn main() -> ExitCode {
    grep(&Cli::parse()).to_exit_code()
}

fn grep(options: &Cli) -> CliResult {
    match Regex::new(&options.pattern) {
        Ok(pattern) => {
            let show_filename = options.files.len() > 1;
            let files_and_results = options
                .files
                .iter()
                .map(|file| (file, Path::new(&file).grep(&pattern)));
            if !options.quiet {
                for (file, result) in files_and_results.clone() {
                    print_result(show_filename, file, &result);
                }
            }
            let results = files_and_results.map(|(_, result)| result);
            let matches = has_matches(&mut results.clone());

            if options.quiet && matches {
                CliResult::Selected
            } else if has_errors(results) {
                CliResult::ErrorOccurred
            } else if matches {
                CliResult::Selected
            } else {
                CliResult::NoneSelected
            }
        }
        Err(err) => {
            eprintln!("{err}");
            CliResult::ErrorOccurred
        }
    }
}

fn print_result(show_filename: bool, file: &str, result: &io::Result<Vec<String>>) {
    match result {
        Ok(matches) => {
            for line in matches {
                println!(
                    "{}{line}",
                    string_utils::str_when(show_filename, &format!("{file}:"))
                )
            }
        }
        Err(err) => eprintln!("grep: {file}: {err}"),
    }
}

fn has_errors<T, E>(results: impl Iterator<Item = Result<T, E>>) -> bool {
    results.filter(|result| result.is_err()).count() > 0
}

fn has_matches<T, E>(results: &mut impl Iterator<Item = Result<Vec<T>, E>>) -> bool {
    results.any(|result| match result {
        Ok(res) => !res.is_empty(),
        Err(_) => false,
    })
}
