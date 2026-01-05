use std::time::{Duration, SystemTime, UNIX_EPOCH};

use chrono::prelude::*;
use rand::prelude::*;
use uuid::Uuid;

use crate::constants::*;

#[derive(Debug, Clone)]
pub enum ColumnKind {
    Uuid,
    Name,
    Surname,
    I64,
    U64,
    F64,
    I32,
    U32,
    F32,
    I16,
    U16,
    I8,
    U8,
    Bool,
    Date,
    DateTime,
}

pub trait GenerateRandomStringValue {
    fn generate_random_string_value(&self) -> String;
}

impl GenerateRandomStringValue for ColumnKind {
    fn generate_random_string_value(&self) -> String {
        let mut rng = rand::rng();

        fn get_random_timestamp(rng: &mut ThreadRng) -> SystemTime {
            UNIX_EPOCH
                + Duration::from_secs(rng.random_range(MIN_UNIX_TIMESTAMP..MAX_UNIX_TIMESTAMP))
        }

        match self {
            ColumnKind::Uuid => Uuid::new_v4().to_string(),
            ColumnKind::Name => FIRST_NAMES.choose(&mut rng).unwrap().to_string(),
            ColumnKind::Surname => SECOND_NAMES.choose(&mut rng).unwrap().to_string(),
            ColumnKind::I64 => rng.random::<i64>().to_string(),
            ColumnKind::U64 => rng.random::<u64>().to_string(),
            ColumnKind::F64 => rng.random::<f64>().to_string(),
            ColumnKind::I32 => rng.random::<i32>().to_string(),
            ColumnKind::U32 => rng.random::<u32>().to_string(),
            ColumnKind::F32 => rng.random::<f32>().to_string(),
            ColumnKind::I16 => rng.random::<i16>().to_string(),
            ColumnKind::U16 => rng.random::<u16>().to_string(),
            ColumnKind::I8 => rng.random::<i8>().to_string(),
            ColumnKind::U8 => rng.random::<u8>().to_string(),
            ColumnKind::Bool => rng.random::<bool>().to_string(),
            ColumnKind::Date => DateTime::<Utc>::from(get_random_timestamp(&mut rng))
                .date_naive()
                .to_string(),
            ColumnKind::DateTime => {
                DateTime::<Utc>::from(get_random_timestamp(&mut rng)).to_string()
            }
        }
    }
}
