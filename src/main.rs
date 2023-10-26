use clap::{Parser, ValueEnum};

fn main() {
    let args = Args::parse();

    let out = match args.p_type {
        PasswordType::Pin => "pin",
        PasswordType::Random => "random",
        PasswordType::Memorable => "memorable",
    };

    println!("type: {out} length: {}", args.length);
}

/// A random password generator
#[derive(Parser)]
#[command(author, version, about, arg_required_else_help = true)]
struct Args {
    /// Password length
    #[arg(default_value_t = 16)]
    length: u32,

    /// Password type
    #[arg(
        short = 'p',
        long = "password-type",
        value_name = "TYPE",
        value_enum,
        default_value_t
    )]
    p_type: PasswordType,

    /// Include numbers
    #[arg(short = 'n', long)]
    numbers: bool,

    /// Include special symbols
    #[arg(short = 's', long)]
    symbols: bool,

    /// Include capitalized letters
    #[arg(short = 'c', long)]
    caps: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Default)]
enum PasswordType {
    #[default]
    Random,
    Pin,
    Memorable,
}
