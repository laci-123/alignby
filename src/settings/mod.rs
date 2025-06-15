use clap::Parser;
use clap::builder::NonEmptyStringValueParser;


#[derive(Debug)]
#[derive(Parser)]
#[command(version, about)]
pub struct Settings {
    /// align on the next non-whitespace character after the delimiter
    #[arg(short, long)]
    pub after: bool,

    /// the string around which to align the lines
    #[arg(value_parser = NonEmptyStringValueParser::new())]
    pub delimiter: String,
}


pub fn parse_from_cmd_line_args() -> Settings {
    Settings::parse()
}
