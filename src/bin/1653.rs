#![allow(dead_code, unused)]

use rand::Rng;

impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let mut count_b = 0;
        let mut res = 0;
        for &c in s.as_bytes() {
            if c == b'b' {
                count_b += 1;
            } else {
                if res + 1 < count_b {
                    res += 1;
                } else {
                    res = count_b;
                }
            }
        }
        res
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::minimum_deletions("aababbab".to_string()), 2);
    assert_eq!(Solution::minimum_deletions("bbaaaaabb".to_string()), 2);
    assert_eq!(Solution::minimum_deletions("bb".to_string()), 0);
    assert_eq!(Solution::minimum_deletions("aa".to_string()), 0);

    let s: String = (0..1_000_000)
        .map(|_| {
            if rand::thread_rng().gen_bool(0.5) {
                'a'
            } else {
                'b'
            }
        })
        .collect();

    let start = std::time::Instant::now();
    Solution::minimum_deletions(s);
    let duration = start.elapsed();

    println!("Time elapsed: {:?}", duration);
}
