use std::str::FromStr;

use super::ColumnKind;
use crate::constants::*;

#[derive(Debug, Clone)]
pub struct Column {
    pub kind: ColumnKind,
    pub name: String,
    pub nillable: bool,
}

impl FromStr for Column {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, kind) = s.split_once(':').ok_or(HELP_MESSAGE)?;

        let nillable = kind.ends_with(NILLABLE_CHAR);
        let kind = kind.strip_suffix(NILLABLE_CHAR).unwrap_or(kind);

        let kind = match kind {
            "uuid" => ColumnKind::Uuid,
            "name" => ColumnKind::Name,
            "surname" => ColumnKind::Surname,
            "i64" => ColumnKind::I64,
            "u64" => ColumnKind::U64,
            "f64" => ColumnKind::F64,
            "i32" => ColumnKind::I32,
            "u32" => ColumnKind::U32,
            "f32" => ColumnKind::F32,
            "i16" => ColumnKind::I16,
            "u16" => ColumnKind::U16,
            "i8" => ColumnKind::I8,
            "u8" => ColumnKind::U8,
            "bool" => ColumnKind::Bool,
            "date" => ColumnKind::Date,
            "datetime" => ColumnKind::DateTime,
            _ => return Err(HELP_MESSAGE.to_string()),
        };

        Ok(Self {
            kind,
            name: name.to_string(),
            nillable,
        })
    }
}
