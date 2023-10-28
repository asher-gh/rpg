// https://www.uic.edu/apps/strong-password/
// ----------------------------- Bonuses ------------------------------
// | Rule                                 | Type      | Rate          |
// | ------------------------------------ | --------- | ------------- |
// | Length                               | Flat      | +(n*4)        |
// | Uppercase letters                    | Cond/Incr | +((len-n)*2)  |
// | Lowercase letters                    | Cond/Incr | +((len-n)*2)  |
// | Numbers                              | Cond      | +(n*4)        |
// | Symbols                              | Flat      | +(n*6)        |
// | Middle numbers or symbols            | Flat      | +(n*2)        |
// | Requirements                         | Flat      | +(n*2)        |
// ---------------------------- Penalties -----------------------------
// | Rule                                 | Type      | Rate          |
// | ------------------------------------ | --------- | ------------- |
// | Letters only                         | Flat      | -n            |
// | Numbers only                         | Flat      | -n            |
// | Repeat Characters (case insensitive) | Comp      | -             |
// | Consecutive uppercase letters        | Flat      | -(n*2)        |
// | Consecutive lowercase letters        | Flat      | -(n*2)        |
// | Consecutive numbers                  | Flat      | -(n*2)        |
// | Sequential letters (3+)              | Flat      | -(n*3)        |
// | Sequential numbers (3+)              | Flat      | -(n*3)        |
// | Sequential symbols (3+)              | Flat      | -(n*3)        |
// --------------------------------------------------------------------
//

#![allow(unused)]

#[derive(Default)]
pub struct AnalyzedPW {
    password: String,
    length: usize,
    uppercase: usize,
    lowercase: usize,
    numbers: usize,
    symbols: usize,
    middle_nums_symbols: usize,
    requirements: usize,
    letters_only: bool,
    numbers_only: bool,
    repeat_chars_penalty: usize,
    cnsctv_uppercase: usize,
    cnsctv_lowercase: usize,
    cnsctv_numbers: usize,
    seq_letters: usize,
    seq_numbers: usize,
    seq_symbols: usize,
}

impl AnalyzedPW {
    pub fn new(pw: String) -> Self {
        let AnalyzedPW {
            mut uppercase,
            mut lowercase,
            mut numbers,
            mut symbols,
            mut middle_nums_symbols,
            mut requirements,
            mut letters_only,
            mut numbers_only,
            mut repeat_chars_penalty,
            mut cnsctv_uppercase,
            mut cnsctv_lowercase,
            mut cnsctv_numbers,
            mut seq_letters,
            mut seq_numbers,
            mut seq_symbols,
            ..
        } = AnalyzedPW::default();
        let length = pw.len();
        let (mut n_repeat_char, mut n_unique_char) = (0usize, 0usize);

        let mut prev_char: Option<char> = None;

        for (i, c) in pw.chars().enumerate() {
            match c {
                'A'..='Z' => {
                    if let Some(prv) = prev_char {
                        if ('A'..='Z').contains(&prv) {
                            cnsctv_uppercase += 1;
                        }
                    }
                    uppercase += 1;
                }
                'a'..='z' => {
                    if let Some(prv) = prev_char {
                        if ('a'..='z').contains(&prv) {
                            cnsctv_lowercase += 1;
                        }
                    }
                    lowercase += 1;
                }
                '0'..='9' => {
                    if let Some(prv) = prev_char {
                        if ('0'..='9').contains(&prv) {
                            cnsctv_numbers += 1;
                        }
                    }
                    if i != 0 && i != length - 1 {
                        middle_nums_symbols += 1;
                    }
                    numbers += 1;
                }
                // assuming all other characters as symbols
                _ => {
                    if i != 0 && i != length - 1 {
                        middle_nums_symbols += 1;
                    }
                    symbols += 1;
                }
            }

            if let Some(prv) = prev_char {
                let prv = prv.to_ascii_lowercase();
                let c = c.to_ascii_lowercase();

                if c as u8 - prv as u8 == 1 {
                    match c {
                        'a'..='z' => seq_letters += if seq_letters == 0 { 2 } else { 1 },
                        '0'..='9' => seq_numbers += if seq_numbers == 0 { 2 } else { 1 },
                        _ => seq_symbols += 1,
                    }
                }
            }

            // Repeating characters penalty
            let mut char_exists = false;
            for (j, char) in pw.chars().enumerate() {
                if c == char && i != j {
                    char_exists = true;
                    /*
                    Calculate icrement deduction based on proximity to identical characters
                    Deduction is incremented each time a new match is discovered
                    Deduction amount is based on total password length divided by the
                    difference of distance between currently selected match
                    */
                    repeat_chars_penalty += length / j.abs_diff(i)
                }
            }
            if char_exists {
                n_repeat_char += 1;
                n_unique_char = length - n_repeat_char;
                repeat_chars_penalty = if n_unique_char > 0 {
                    f64::ceil(repeat_chars_penalty as f64 / n_unique_char as f64) as usize
                } else {
                    repeat_chars_penalty
                }
            }

            prev_char = Some(c);
        }

        // Letters only
        // numbers_only,
        match length {
            x if x == (uppercase + lowercase) => letters_only = true,
            y if y == numbers => numbers_only = true,
            _ => {}
        }

        // Requirements
        if length >= 8 {
            requirements += 1
        }

        [uppercase, lowercase, numbers, symbols]
            .iter()
            .for_each(|x| {
                if *x > 0 {
                    requirements += 1
                }
            });

        AnalyzedPW {
            password: pw,
            length,
            uppercase,
            lowercase,
            numbers,
            symbols,
            middle_nums_symbols,
            requirements,
            letters_only,
            numbers_only,
            repeat_chars_penalty,
            cnsctv_uppercase,
            cnsctv_lowercase,
            cnsctv_numbers,
            seq_letters,
            seq_numbers,
            seq_symbols,
        }
    }

    pub fn score(&self) -> f64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref APWS: Vec<AnalyzedPW> = vec![AnalyzedPW {
            password: String::from("123456"),
            length: 6,
            uppercase: 0,
            lowercase: 0,
            numbers: 6,
            symbols: 0,
            middle_nums_symbols: 4,
            requirements: 1,
            letters_only: false,
            numbers_only: true,
            repeat_chars_penalty: 0,
            cnsctv_uppercase: 0,
            cnsctv_lowercase: 0,
            cnsctv_numbers: 5,
            seq_letters: 0,
            seq_numbers: 6,
            seq_symbols: 0,
        }];
    }

    #[test]
    fn test_pw_analyzer() {
        for expected in APWS.iter() {
            let got = AnalyzedPW::new(expected.password.clone());
            assert_eq!(expected.length, got.length);
            assert_eq!(expected.uppercase, got.uppercase);
            assert_eq!(expected.lowercase, got.lowercase);
            assert_eq!(expected.numbers, got.numbers);
            assert_eq!(expected.symbols, got.symbols);
            assert_eq!(expected.middle_nums_symbols, got.middle_nums_symbols);
            assert_eq!(expected.requirements, got.requirements);
            assert_eq!(expected.letters_only, got.letters_only);
            assert_eq!(expected.numbers_only, got.numbers_only);
            assert_eq!(expected.repeat_chars_penalty, got.repeat_chars_penalty);
            assert_eq!(expected.cnsctv_uppercase, got.cnsctv_uppercase);
            assert_eq!(expected.cnsctv_lowercase, got.cnsctv_lowercase);
            assert_eq!(expected.cnsctv_numbers, got.cnsctv_numbers);
            assert_eq!(expected.seq_letters, got.seq_letters);
            assert_eq!(expected.seq_numbers, got.seq_numbers);
            assert_eq!(expected.seq_symbols, got.seq_symbols);
        }
    }
}
