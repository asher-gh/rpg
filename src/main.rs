use clap::Parser;
use owo_colors::OwoColorize;
use pass::Password;

mod pass;

fn main() {
    let args = Args::parse();
    let (password, entropy) =
        args.p_type
            .gen_pass(args.length, args.numbers, args.symbols, args.caps);

    println!("{}", password.green());
    eprintln!("{}", entropy.yellow());
}

/// A random password generator
#[derive(Parser)]
#[command(author, version, about, arg_required_else_help = true)]
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
