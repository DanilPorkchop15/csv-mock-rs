use rand::prelude::*;

use crate::args::Args;
use crate::column::{Column, GenerateRandomStringValue};

type ColumnMapperFn = fn(&Column) -> String;

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

fn columns_mapper(args: &Args, mapper_fn: ColumnMapperFn) -> String {
    args.columns
        .iter()
        .map(mapper_fn)
        .collect::<Vec<_>>()
        .join(&args.column_delimiter)
}
