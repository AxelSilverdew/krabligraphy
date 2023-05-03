//! # Krabligraphy 🦀
//!
//! This is a rather silly crate that is intended for generating text that you can paste into
//! Discord for fun.
//!
//! ### Why's it called 'Krabligraphy'? 🦀
//!
//! It's an inside joke on one of the discord servers that I'm in.

#![warn(clippy::missing_docs_in_private_items)]

use rand::{thread_rng, Rng};

/// Takes a sentence and wraps each individual character in spoiler tags.
pub fn spoilerify(text: &str) -> String {
    text.chars()
        .map(|letter| format!("||{}|| ", letter))
        .collect::<Vec<_>>()
        .join("")
        .trim_end()
        .to_string()
}

/// Takes a sentence and generates a series of emoji that spells out the entire sentence.
pub fn emojify(text: &str) -> String {
    let mut result = String::new();
    let mut rng = thread_rng();

    for char in text.chars() {
        match char {
            'a'..='z' | 'A'..='Z' => result.push_str(&format!(
                ":regional_indicator_{}: ",
                char.to_ascii_lowercase()
            )),
            '0'..='9' => result.push_str(&format!(
                ":{}: ",
                match char {
                    '0' => "zero",
                    '1' => "one",
                    '2' => "two",
                    '3' => "three",
                    '4' => "four",
                    '5' => "five",
                    '6' => "six",
                    '7' => "seven",
                    '8' => "eight",
                    '9' => "nine",
                    _ => unreachable!(),
                }
            )),
            '?' => result.push_str(":question:"),
            '!' => result.push_str(":exclamation:"),
            ' ' => result.push_str("   "),
            '\n' => result.push('\n'),
            _ => result.push(char),
        }
    }
    result.push('\n');
    for char in text.chars() {
        match char {
            ' ' => result.push_str("   "),
            _ => result.push_str(&format!(
                ":woman_gesturing_ok::skin-tone-{}: ",
                rng.gen_range(1..6)
            )),
        }
    }
    String::from(result.trim())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spoilerify() {
        let actual = spoilerify("this is a test");
        assert_eq!(
            "||t|| ||h|| ||i|| ||s|| || || ||i|| ||s|| || || ||a|| || || ||t|| ||e|| ||s|| ||t||",
            actual
        );
    }

    #[test]
    fn test_emojify() {
        let actual = emojify("this is a test");
        assert_eq!(
            ":regional_indicator_t: :regional_indicator_h: :regional_indicator_i: :regional_indicator_s:    :regional_indicator_i: :regional_indicator_s:    :regional_indicator_a:    :regional_indicator_t: :regional_indicator_e: :regional_indicator_s: :regional_indicator_t:", 
            actual
        );
    }
}
