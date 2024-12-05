#![allow(dead_code)]
use std::{collections::HashMap, time::{SystemTime, UNIX_EPOCH}};


impl Solution {
    pub fn count_arrangement_stupid(n: i32) -> i32 {
        fn backtrack(position: i32, n: i32, used: &mut Vec<bool>) -> i32 {
            // Base case: all positions filled
            if position > n {
                return 1;
            }

            let mut total = 0;

            // Try placing each number at this position
            for num in 1..=n {
                if !used[num as usize] && (num % position == 0 || position % num == 0) {
                    // Mark the number as used
                    used[num as usize] = true;

                    // Recurse to the next position
                    total += backtrack(position + 1, n, used);

                    // Backtrack: unmark the number
                    used[num as usize] = false;
                }
            }

            total
        }

        // Track which numbers are used
        let mut used = vec![false; (n + 1) as usize];

        // Start backtracking from position 1
        backtrack(1, n, &mut used)
    }
}

impl Solution {
    pub fn count_arrangement(n: i32) -> i32 {
        let mut memo = HashMap::new();
        fn dfs(mask: i32, pos: i32, n: i32, memo: &mut HashMap<(i32, i32), i32>) -> i32 {
            if mask == (1 << n) - 1 {
                return 1;
            }
            if let Some(&cached) = memo.get(&(mask, pos)) {
                return cached;
            }
            let mut res = 0;
            for i in (1..n + 1).rev() {
                if mask & (1 << i - 1) == 0 && (i % pos == 0 || pos % i == 0) {
                    res += dfs(mask | (1 << i - 1), pos - 1, n, memo);
                }
            }
            memo.insert((mask, pos), res);
            res
        }
        dfs(0, n, n, &mut memo)
    }
}

struct Solution;

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    dbg!(Solution::count_arrangement_stupid(17));
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    dbg!(end - start);
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    dbg!(Solution::count_arrangement(17));
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    dbg!(end - start);
}
