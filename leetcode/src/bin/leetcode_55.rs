#![allow(dead_code)]

impl Solution {
    pub fn find(a: &Vec<i32>, pos: usize, visited: &mut Vec<bool>) -> bool {
        if pos == a.len() - 1 {
            return true;
        }
        if visited[pos] {
            return false;
        }
        visited[pos] = true;
        for i in 1..=a[pos] as usize {
            if Solution::find(&a, pos + i, visited) {
                return true;
            }
            visited[pos] = true;
        }
        false
    }

    pub fn can_jump_stupid(a: Vec<i32>) -> bool {
        let mut visited = vec![false; a.len()];
        let result = Solution::find(&a, 0, &mut visited);
        result
    }

    pub fn can_jump(a: Vec<i32>) -> bool {
        let mut gas = 0;
        for n in a {
            if gas < 0 {
                return false;
            } else if n > gas {
                gas = n
            }
            gas -= 1;
        }
        true
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true);
    assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false);
    assert_eq!(
        Solution::can_jump(vec![
            2, 3, 1, 1, 4, 2, 1, 5, 3, 2, 1, 0, 4, 2, 3, 1, 1, 2, 3, 1
        ]),
        true
    );
}
