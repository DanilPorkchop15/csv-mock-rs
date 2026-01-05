mod kind;
mod parser;

pub use kind::*;

#[derive(Debug, Clone)]
pub struct Column {
    pub kind: ColumnKind,
    pub name: String,
    pub nillable: bool,
}
