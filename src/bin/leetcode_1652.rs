#![allow(dead_code)]

impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let len = code.len();
        let mut res = vec![0; len];
        match k {
            0 => {}
            _ if k < 0 => {
                for i in 0..len {
                    let i = i as i32;
                    let mut sum = 0;
                    for j in i + k..=i - 1 {
                        sum += code[((j + len as i32) as usize) % len];
                    }
                    res[i as usize] = sum;
                }
            }
            _ => {
                for i in 0..len {
                    let mut sum = 0;
                    for j in i + 1..=i + k as usize {
                        sum += code[j % len];
                    }
                    res[i] = sum;
                }
            }
        };
        res
    }
}

struct Solution;

fn main() {
    dbg!(Solution::decrypt(vec![5, 7, 1, 4], 3));
    dbg!(Solution::decrypt(vec![1, 2, 3, 4], 0));
    dbg!(Solution::decrypt(vec![2, 4, 9, 3], -2));
}
