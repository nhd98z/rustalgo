#[allow(dead_code)]

struct Solution;

fn main() {
    dbg!(Solution::count_arrangement(1));
    dbg!(Solution::count_arrangement(2));
    dbg!(Solution::count_arrangement(3));
    dbg!(Solution::count_arrangement(4));
}

impl Solution {
    fn sol(res: &mut i32, n: i32, pos: i32, used: &mut Vec<bool>) {
        if pos > n {
            *res += 1;
        };
        for i in 1..=n {
            if used[i as usize] == false && (i % pos == 0 || pos % i == 0) {
                used[i as usize] = true;
                Self::sol(res, n, pos + 1, used);
                used[i as usize] = false;
            }
        }
    }

    pub fn count_arrangement(n: i32) -> i32 {
        let mut res = 0;
        Self::sol(&mut res, n, 1, &mut vec![false; n as usize + 1]);
        res
    }
}
