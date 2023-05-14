use clap::{Parser, Subcommand, ValueEnum};

use crate::examples;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Runs a .rx file
    Run { name: String },

    /// Runs an included example, or lists all examples.
    Example { name: IncludedApps },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum IncludedApps {
    /// Run FizzBuzz
    FizzBuzz,

    /// Run stdio
    Stdio,

    /// Run the CLI interpreter
    Interpreter,
}

pub fn parse_cli() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run { name } => {
            // Resolve it to a file
            let path = std::path::Path::new(name);
            let path = if path.is_absolute() {
                path.to_path_buf()
            } else {
                let mut path = std::env::current_dir().unwrap();
                path.push(name);
                path
            };

            // Check if there's a file
            if !path.exists() {
                println!("File not found: {}", path.display());
                return;
            }

            // Read the file
            let input = std::fs::read_to_string(path).unwrap();
            super::execute(input);
        }
        Commands::Example { name } => {
            match name {
                IncludedApps::FizzBuzz => {
                    examples::fizzbuzz();
                }
                IncludedApps::Stdio => {
                    examples::stdio();
                },
                IncludedApps::Interpreter => {
                    examples::interpreter();
                }
            }
        }
    }
}