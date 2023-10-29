use clap::ValueEnum;
use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use regex::Regex;
use std::fs;
lazy_static! {
    static ref WORD_LIST: String = fs::read_to_string("./assets/eff_large_wordlist.txt").unwrap();
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Default)]
pub enum Password {
    #[default]
    Random,
    Pin,
    Memorable,
}

#[allow(unused)]
impl Password {
    /// Generates password based on the type.
    pub fn gen_pass(&self, len: usize, nums: bool, symbols: bool, caps: bool) -> String {
        let mut rng = thread_rng();
        let upper_case: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let lower_case: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
        let digits: &[u8] = b"0123456789";
        let special: &[u8] = b"!\"#$%&'*+,./:;=?@\\^`|~";

        let mut alphabet: Vec<u8> = match self {
            Self::Random => lower_case.into(),
            Self::Pin => digits.into(),
            Self::Memorable => {
                return (0..len)
                    .map(|_| Self::gen_phrase())
                    .collect::<Vec<String>>()
                    .join("-");
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

        password
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

    /// Simulates a dice roll
    fn roll_dice() -> u8 {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=6)
    }

    /// Calculates entropy of a password based on log2(symbols^length)
    pub fn entropy(possible_symboles: usize, length: usize) -> f64 {
        f64::log2(f64::powf(possible_symboles as f64, length as f64))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_password_length() {
        for len in 0..50 {
            // RANDOM
            let pw = Password::Random;

            // test length for all flag options
            for i in 0..8 {
                let mut x = i;

                let a: &[bool] = &(0..3)
                    .map(|_| {
                        let y = x & 1;
                        x >>= 1;
                        y == 1
                    })
                    .collect::<Vec<bool>>();

                let password = pw.gen_pass(len, a[0], a[1], a[2]);
                assert_eq!(password.len(), len);
            }

            // PIN
            let pw = Password::Pin;
            let password = pw.gen_pass(len, false, false, false); // flagless
            assert_eq!(password.len(), len);

            // MEMORABLE
            let pw = Password::Memorable;
            let password = pw.gen_pass(len, false, false, false); // flagless
            if len == 0 {
                assert_eq!(password.len(), len);
            } else {
                let words: Vec<&str> = password.split("-").collect();
                assert_eq!(words.len(), len);
            }
        }
    }
}
