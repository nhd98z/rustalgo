#![allow(dead_code, unused)]

use std::{
    cmp::min,
    time::{SystemTime, UNIX_EPOCH},
};

impl Solution {
    fn dfs(matchsticks: &Vec<i32>, sides: &mut Vec<i32>, pos: i32, target: i32) -> bool {
        if pos == matchsticks.len() as i32 {
            return sides[0] == sides[1] && sides[1] == sides[2] && sides[2] == sides[3];
        }
        for i in 0..4 {
            if sides[i] + matchsticks[pos as usize] > target {
                continue;
            }
            if sides[..i].iter().any(|&side| side == sides[i]) {
                continue;
            }
            sides[i] += matchsticks[pos as usize];
            if Self::dfs(&matchsticks, sides, pos + 1, target) {
                return true;
            }
            sides[i] -= matchsticks[pos as usize];
        }
        false
    }

    pub fn makesquare(matchsticks: Vec<i32>) -> bool {
        let sum: i32 = matchsticks.iter().sum();
        if (sum % 4 != 0) {
            return false;
        }
        let target = sum / 4;
        if matchsticks.iter().any(|&ms| ms > target) {
            return false;
        }
        let mut matchsticks = matchsticks;
        matchsticks.sort();
        matchsticks.reverse();
        let mut sides = vec![0; 4];
        Self::dfs(&matchsticks, &mut sides, 0, target)
    }
}

struct Solution;

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    assert!(Solution::makesquare(vec![1, 1, 1, 1]) == true);
    assert!(Solution::makesquare(vec![1, 1, 2, 2, 2]) == true);
    assert!(Solution::makesquare(vec![3, 3, 4, 3, 3]) == false);
    assert!(Solution::makesquare(vec![3, 3, 3, 3, 4, 2, 1]) == false);
    assert!(Solution::makesquare(vec![10, 6, 5, 5, 5, 3, 3, 3, 2, 2, 2, 2]) == true);
    assert!(Solution::makesquare(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 5, 4, 3, 2, 1]) == false);
    assert!(Solution::makesquare(vec![5, 5, 5, 5, 4, 4, 4, 4, 3, 3, 3, 3]) == true);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    dbg!(end - start);
}
