#![allow(dead_code)]

impl Solution {
    pub fn rotate(m: &mut Vec<Vec<i32>>) {
        let n = m.len();
        for i in 0..n {
            for j in i + 1..n {
                let temp = m[j][i];
                m[j][i] = m[i][j];
                m[i][j] = temp;
            }
        }
        for i in 0..n {
            m[i].reverse();
        }
    }
}

struct Solution;

fn main() {
}
