pub const FIRST_NAMES: [&str; 4] = ["Danil", "Egor", "Dima", "Andrew"];
pub const SECOND_NAMES: [&str; 4] = ["Kostin", "Lyadskiy", "Prudnikov", "Eremenko"];

pub const MIN_UNIX_TIMESTAMP: u64 = 1_000_000_000;
pub const MAX_UNIX_TIMESTAMP: u64 = 2_000_000_000;

pub const NILLABLE_CHAR: char = '%';

pub const HELP_MESSAGE: &str = "Available types: uuid name, surname, i64, u64, f64, i32, u32, f32, i16, u16, i8, u8, bool, date, datetime\
\n\nColumn input format: <name>:<type><% - nillable field, optional>";
