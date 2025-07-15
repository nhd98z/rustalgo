macro_rules! s {
    ($str:expr) => {
        String::from($str)
    };
}

impl Solution {
    pub fn is_valid(word: String) -> bool {
        // Example validation without regex
        if word.len() < 3 {
            return false;
        }

        let mut has_vowel = false;
        let mut has_consonant = false;

        for ch in word.chars() {
            match ch {
                'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => {
                    has_vowel = true;
                }
                c if c.is_alphabetic() => {
                    has_consonant = true;
                }
                c if c.is_ascii_digit() => {
                    // digits are allowed
                }
                _ => {
                    // invalid character
                    return false;
                }
            }
        }

        has_vowel && has_consonant
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::is_valid(s!("234Adas")), true);
}
