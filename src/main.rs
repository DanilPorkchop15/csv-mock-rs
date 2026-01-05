use std::fs;
mod args;
mod column;
mod constants;
mod generator;

use args::Args;
use clap::Parser;
use generator::{build_header, build_row};

fn main() {
    let args = Args::parse();
    let mut rows = vec![];

    if !args.headless {
        rows.push(build_header(&args));
    }

    for _ in 0..args.count {
        rows.push(build_row(&args));
    }

    let output = rows.join(&args.row_delimiter);

    if let Some(output_file) = args.output_file {
        fs::write(output_file, output).unwrap();
    } else {
        println!("{}", output);
    }
}
