impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        let mut left = 0;
        let n = colors.len() as usize;
        let k = k as usize;
        for right in 0..n + k - 1 {
            if right > 0 && colors[right % n] == colors[(right - 1) % n] {
                left = right;
            }
            if right - left + 1 >= k {
                count += 1;
            }
        }
        count
    }
}

struct Solution;

fn main() {
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "1");
    }
    assert_eq!(
        Solution::number_of_alternating_groups(vec![0, 1, 0, 1, 0], 3),
        3
    );
    assert_eq!(
        Solution::number_of_alternating_groups(vec![0, 1, 0, 0, 1, 0, 1], 6),
        2
    );
}
