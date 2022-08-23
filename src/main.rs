use std::io::{self, BufRead};

use clap::{Parser, Subcommand};

/// Simple stats calculator
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Calculates the mean of the data
    Mean {
    },
    /// Calculates the sum of the data
    Sum {
    },
    /// Calculates the maximum of the data
    Max {
    },
    /// Calculates the minimum of the data
    Min {
    },
}

fn get_single_column() -> Vec<f64> {
    let stdin = io::stdin();
    let mut data: Vec<f64> = vec![];
    for line in stdin.lock().lines() {
        let line_text = match line {
            Ok(line) => line,
            Err(err) => panic!("IO error: {}", err),
        };
        data.push(line_text.parse::<f64>()
            .expect(&format!("ERROR: Could not parse '{}' as an f64", line_text)));
    }
    data
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Mean{} => {
            let data = get_single_column();
            let sum: f64 = data.iter().sum();
            let count = data.len();
            let mean = sum / count as f64;
            println!("{}", mean);
        },
        Commands::Sum{} => {
            let data = get_single_column();
            let sum: f64 = data.iter().sum();
            println!("{}", sum);
        },
        Commands::Max{} => {
            let data = get_single_column();
            let max: f64 = data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
            println!("{}", max);
        },
        Commands::Min{} => {
            let data = get_single_column();
            let min: f64 = data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
            println!("{}", min);
        },
    }
}
