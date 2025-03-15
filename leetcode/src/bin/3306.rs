use std::collections::HashMap;

macro_rules! s {
    ($str:expr) => {
        String::from($str)
    };
}

impl Solution {
    fn count_at_least_k(word: &str, k: i32) -> i64 {
        let mut l = 0;
        let mut r = 0;
        let mut cnt = 0;
        let n = word.len();
        let arr: Vec<char> = word.chars().collect();
        let mut vowels = HashMap::from([('a', 0), ('e', 0), ('i', 0), ('o', 0), ('u', 0)]);
        let mut num_consonants = 0;
        while l < n || r < n {
            // dbg!(&vowels);
            if vowels.values().all(|&v| v >= 1) && num_consonants >= k {
                cnt += n - r + 1;
                if let Some(v) = vowels.get_mut(&arr[l]) {
                    *v -= 1;
                } else {
                    num_consonants -= 1;
                }
                l += 1;
            } else {
                if r == n {
                    break;
                }
                if let Some(v) = vowels.get_mut(&arr[r]) {
                    *v += 1;
                } else {
                    num_consonants += 1;
                }
                r += 1;
            }
        }
        cnt as _
    }

    pub fn count_of_substrings(word: String, k: i32) -> i64 {
        Self::count_at_least_k(&word, k) - Self::count_at_least_k(&word, k + 1)
    }
}

struct Solution;

fn main() {
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "1");
    }
    assert_eq!(Solution::count_of_substrings(s!("aeioqq"), 1), 0);
    assert_eq!(Solution::count_of_substrings(s!("aeiou"), 0), 1);
    assert_eq!(Solution::count_of_substrings(s!("ieaouqqieaouqq"), 1), 3);
    assert_eq!(Solution::count_of_substrings(s!("aeueio"), 0), 1);
}
