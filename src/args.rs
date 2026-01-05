use clap::Parser;

use crate::column::Column;
use crate::constants::HELP_MESSAGE;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = HELP_MESSAGE)]
pub struct Args {
    #[arg(short, long, default_value = "1")]
    pub count: u32,

    #[arg(short = 'C', long = "col", value_name = "<NAME>:<TYPE><%>", action = clap::ArgAction::Append)]
    pub columns: Vec<Column>,

    #[arg(long, default_value = "\t")]
    pub column_delimiter: String,

    #[arg(long, default_value = "\n")]
    pub row_delimiter: String,

    #[arg(short = 'H', long)]
    pub headless: bool,

    #[arg(short, long)]
    pub output_file: Option<String>,
}
