use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {
    let roman_values: HashMap<char, i32> = [
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]
    .iter()
    .cloned()
    .collect();

    s.chars()
        .rev()
        .fold((0, 0), |(acc, prev_value), c| {
            let current_value = roman_values[&c];
            if current_value < prev_value {
                (acc - current_value, current_value)
            } else {
                (acc + current_value, current_value)
            }
        })
        .0
}

fn main() {
    dbg!(roman_to_int("MCMXCIV".to_string()));
}
