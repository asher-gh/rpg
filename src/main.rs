use clap::{Parser, ValueEnum};
use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use regex::Regex;
use std::fs;

lazy_static! {
    static ref WORD_LIST: String = fs::read_to_string("./assets/eff_large_wordlist.txt").unwrap();
}

fn main() {
    let args = Args::parse();
    let (password, entropy) =
        args.p_type
            .gen_pass(args.length, args.numbers, args.symbols, args.caps);

    println!("{password}");
    eprintln!("{entropy}");
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

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Default)]
enum Password {
    #[default]
    Random,
    Pin,
    Memorable,
}

impl Password {
    /// Generates password based on the type.
    fn gen_pass(&self, len: usize, nums: bool, symbols: bool, caps: bool) -> (String, f64) {
        let mut rng = thread_rng();
        let upper_case: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let lower_case: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
        let digits: &[u8] = b"0123456789";
        let special: &[u8] = b"!\"#$%&'*+,./:;=?@\\^`|~";

        let mut alphabet: Vec<u8> = match self {
            Self::Random => lower_case.into(),
            Self::Pin => digits.into(),
            Self::Memorable => {
                let password = (0..len)
                    .map(|_| Self::gen_phrase())
                    .collect::<Vec<String>>()
                    .join("-");

                let strength = Self::entropy(lower_case.len() + 1, password.len());
                return (password, strength);
            }
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

        let password: String = (0..len as usize)
            .map(|_| {
                let idx = rng.gen_range(0..alphabet.len());
                alphabet[idx] as char
            })
            .collect();

        let pl = f64::powf(alphabet.len() as f64, password.len() as f64);
        let strength = Self::entropy(alphabet.len(), password.len());

        (password, strength)
    }

    /// Picks a random phrase using the EFF's dice word list
    /// https://www.eff.org/dice
    fn gen_phrase() -> String {
        let idx: String = (0..5)
            .map(|_| char::from_digit(Self::roll_dice() as u32, 10).unwrap())
            .collect::<Vec<char>>()
            .iter()
            .collect();

        let re = Regex::new(&format!(r"(?m)^{idx}\s(?<phrase>\w+)$")).unwrap();

        if let Some(caps) = re.captures(&WORD_LIST) {
            return caps["phrase"].to_string();
        };

        String::default()
    }

    /// Simulates dice roll
    fn roll_dice() -> u8 {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=6)
    }

    /// Calculate entropy of a password based on log2(symbols^length)
    fn entropy(possible_symboles: usize, length: usize) -> f64 {
        f64::log2(f64::powf(possible_symboles as f64, length as f64))
    }
}
