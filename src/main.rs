use clap::{Parser, ValueEnum};
use rand::{thread_rng, Rng};

// TODO: Memorable
// TODO: Strength

fn main() {
    let args = Args::parse();
    let out = args
        .p_type
        .gen_pass(args.length, args.numbers, args.symbols, args.caps);

    println!("{out}");
}

/// A random password generator
#[derive(Parser)]
#[command(author, version, about, arg_required_else_help = true)]
struct Args {
    /// Password length
    #[arg(default_value_t = 16)]
    length: u8,

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

impl PasswordType {
    #[allow(unused)]
    fn gen_pass(&self, len: u8, nums: bool, symbols: bool, caps: bool) -> String {
        let mut rng = thread_rng();
        let upper_case: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let lower_case: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
        let digits: &[u8] = b"0123456789";
        let special: &[u8] = b"!\"#$%&'*+,./:;=?@\\^`|~";
        // let minus = "-";

        let mut alphabet: Vec<u8> = match self {
            PasswordType::Random => lower_case.into(),
            PasswordType::Pin => digits.into(),
            // TODO: memorable
            _ => vec![],
        };

        if self == &Self::Random {
            if caps {
                alphabet = [&alphabet, upper_case].concat();
            }
            if nums {
                alphabet = [&alphabet, digits].concat();
            }
            if symbols {
                alphabet = [&alphabet, special].concat();
            }
        }

        dbg!(&std::str::from_utf8(&alphabet));

        let password: String = (0..len as usize)
            .map(|_| {
                let idx = rng.gen_range(0..alphabet.len());
                alphabet[idx] as char
            })
            .collect();

        password
    }
}
