impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let roman_values = vec![
            ("M", 1000),
            ("CM", 900),
            ("D", 500),
            ("CD", 400),
            ("C", 100),
            ("XC", 90),
            ("L", 50),
            ("XL", 40),
            ("X", 10),
            ("IX", 9),
            ("V", 5),
            ("IV", 4),
            ("I", 1),
        ];

        let mut result = String::new();
        let mut num = num;

        for (roman, value) in roman_values.iter() {
            while num >= *value {
                result.push_str(roman);
                num -= value;
            }
        }

        result
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::int_to_roman(3), "III");
    assert_eq!(Solution::int_to_roman(3749), "MMMDCCXLIX");
}
