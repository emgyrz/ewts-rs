use clap::Parser;
use ewts_lib::EwtsConverter;
use std::fmt::Display;

#[derive(clap::ValueEnum, Debug, Clone)]
enum SourceType {
    Ewts,
    Unicode,
}

impl Display for SourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SourceType::Ewts => f.write_str("ewts"),
            SourceType::Unicode => f.write_str("unicode"),
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Type of input symbols
    #[arg(short, long, default_value_t = SourceType::Ewts)]
    source_type: SourceType,

    /// String to convert
    #[arg(short, long)]
    input: String,
}

fn main() {
    let args = Args::parse();
    let converter = EwtsConverter::create();
    let result = converter.ewts_to_unicode(&args.input);
    println!("{}", result);
}
