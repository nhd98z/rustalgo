macro_rules! s {
    ($str:expr) => {
        String::from($str)
    };
}

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s = s.trim();
        if s.is_empty() {
            return 0;
        }

        // Handle sign
        let (s, sign) = match s.chars().next() {
            Some('+') => (&s[1..], 1),
            Some('-') => (&s[1..], -1),
            _ => (s, 1),
        };

        // Extract and parse digits
        let mut result = 0i32;

        for c in s.chars() {
            if !c.is_ascii_digit() {
                break;
            }

            let digit = (c as u8 - b'0') as i32;

            // Check for overflow
            if result > i32::MAX / 10 || (result == i32::MAX / 10 && digit > 7) {
                return if sign == 1 { i32::MAX } else { i32::MIN };
            }

            result = result * 10 + digit;
        }

        result * sign
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::my_atoi(s!("42")), 42);
}
