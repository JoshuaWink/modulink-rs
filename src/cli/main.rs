//! CLI entry point for modulink-rust
//! Supports: run, visualize, doc

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "modulink-cli")]
#[command(about = "ModuLink-Rust CLI tools", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a chain with input context
    Run {
        #[arg(short, long)]
        input: Option<String>,
    },
    /// Visualize a chain as DOT/Graphviz
    Visualize {},
    /// Show documentation/help
    Doc {
        #[arg(short, long)]
        topic: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Run { input } => {
            println!("[CLI] Run chain with input: {:?}", input);
            // TODO: Load chain, parse input, run chain
        }
    }
}
