use std::{error, fs};
mod args;
mod column;
mod constants;
mod generator;

use args::Args;
use clap::Parser;
use generator::{build_header, build_row};

fn run() -> Result<(), Box<dyn error::Error>> {
    let args = Args::parse();
    let mut rows = vec![];

    if !args.headless {
        rows.push(build_header(&args));
    }

    for _ in 0..args.count {
        rows.push(build_row(&args));
    }

    let output = rows.join(&args.row_delimiter);

    if let Some(path) = &args.output_file {
        if let Err(e) = fs::write(path, &output) {
            eprintln!("Failed to write '{}': {}", path, e);
            std::process::exit(1);
        }
    } else {
        println!("{}", output);
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
