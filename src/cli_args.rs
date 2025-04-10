#[allow(unused)] // CommandFactory is not unused, it's needed for #[command(...)]
use clap::{CommandFactory, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
#[command(name = "completion-derive")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Encode {
        /// Data to emojfuscate, pass a dash (-) to read from stdin
        input: String,
        #[arg(short, long, default_value_t = DataType::Text)]
        data_type: DataType,
        /// Will append a line break at the end of the output if set
        #[arg(short, long)]
        line_break: bool,
    },
    Decode {
        /// data to demojfuscate, pass a dash (-) to read from stdin
        input: String,
        #[arg(short, long, default_value_t = DataType::Text)]
        data_type: DataType,
        /// Will append a line break at the end of the output if set
        #[arg(short, long)]
        line_break: bool,
    },
}

#[derive(ValueEnum, Clone, Debug)]
pub enum DataType {
    Text,
    UUID,
    Hexadecimal,
}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stringified = match self {
            DataType::Text => "text",
            DataType::UUID => "uuid",
            DataType::Hexadecimal => "hexadecimal",
        };
        write!(f, "{}", stringified)
    }
}
