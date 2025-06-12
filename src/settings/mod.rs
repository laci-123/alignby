use clap::Parser;


#[derive(Debug)]
#[derive(Parser)]
#[command(version, about)]
pub struct Settings {
    /// insert padding after the delimiter, not before it
    #[arg(short, long)]
    pub after: bool,

    /// the string around which to align the lines
    pub delimiter: String,
}


pub fn parse_from_cmd_line_args() -> Settings {
    Settings::parse()
}
