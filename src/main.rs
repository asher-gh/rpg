use clap::Parser;
use owo_colors::OwoColorize;
use pass::Password;

use crate::scorer::AnalyzedPW;

mod pass;
mod scorer;

fn main() {
    let args = Args::parse();
    let password = args
        .p_type
        .gen_pass(args.length, args.numbers, args.symbols, args.caps);

    let strength = AnalyzedPW::new(password.clone());

    println!("Password: {}", password.green());
    eprintln!("Strength: {}", format!("{}%", strength.score()).yellow());
}

/// A random password generator and strength estimator.
#[derive(Parser)]
#[command(
    author,
    version,
    about,
    arg_required_else_help = true,
    next_line_help = true,
    help_template = "\
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
"
)]
struct Args {
    /// Password length
    #[arg(default_value_t = 16)]
    length: usize,

    /// Password type
    #[arg(
        short = 'p',
        long = "password-type",
        value_name = "TYPE",
        value_enum,
        default_value_t
    )]
    p_type: Password,

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
