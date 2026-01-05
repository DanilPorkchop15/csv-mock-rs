pub const FIRST_NAMES: [&str; 10] = [
    "Danil", "Egor", "Dima", "Andrew", "Alice", "Bob", "Charlie", "Eva", "Sophia", "Liam",
];

pub const SECOND_NAMES: [&str; 10] = [
    "Kostin",
    "Lyadskiy",
    "Prudnikov",
    "Eremenko",
    "Smith",
    "Johnson",
    "Brown",
    "Taylor",
    "Lee",
    "Davis",
];

pub const MIN_UNIX_TIMESTAMP: u64 = 1_000_000_000;
pub const MAX_UNIX_TIMESTAMP: u64 = 2_000_000_000;

pub const NILLABLE_CHAR: char = '%';

pub const HELP_MESSAGE: &str = "Available types: uuid name, surname, i64, u64, f64, i32, u32, f32, i16, u16, i8, u8, bool, date, datetime\
\n\nColumn input format: <name>:<type><% - nillable field, optional>";
