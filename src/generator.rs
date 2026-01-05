use rand::prelude::*;

use crate::args::Args;
use crate::column::{Column, GenerateRandomStringValue};

pub fn build_header(args: &Args) -> String {
    columns_mapper(args, |c| c.name.clone())
}

pub fn build_row(args: &Args) -> String {
    columns_mapper(args, |c| {
        if c.nillable && rand::rng().random::<bool>() {
            return String::new();
        }
        c.kind.generate_random_string_value()
    })
}

fn columns_mapper<F>(args: &Args, f: F) -> String
where
    F: Fn(&Column) -> String,
{
    args.columns
        .iter()
        .map(f)
        .collect::<Vec<_>>()
        .join(&args.column_delimiter)
}
