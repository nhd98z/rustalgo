impl Solution {
    pub fn smallest_number(pattern: String) -> String {
        let n = pattern.len();
        let mut result = vec![0; n + 1];
        let mut curr = 1;
        let chars: Vec<char> = pattern.chars().collect();

        for i in 0..=n {
            if i == n || chars[i] == 'I' {
                let mut k = i as i32;
                while k >= 0 && (k == i as i32 || chars[k as usize] == 'D') {
                    result[k as usize] = curr;
                    curr += 1;
                    k -= 1;
                }
            }
        }

        result
            .into_iter()
            .map(|d| char::from_digit(d as u32, 10).unwrap())
            .collect()
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::smallest_number("IIIDIDDD".to_string()), "123549876");
}
