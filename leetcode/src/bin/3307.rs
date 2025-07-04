impl Solution {
    pub fn kth_character(k: i64, operations: Vec<i32>) -> char {
        let mut pos = k - 1;
        let mut flips = 0;
        for i in (0..operations.len()).rev() {
            if i >= 50 {
                // 2^50 > 10^14, so we can skip large i
                continue;
            }
            let prev_length = 1i64 << i;
            if pos >= prev_length {
                pos = pos - prev_length;
                if operations[i] == 1 {
                    flips += 1;
                }
            }
        }
        (b'a' + (flips % 26)) as char
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::kth_character(
            33354182522397,
            vec![
                0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1,
                0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1,
                1, 0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 1, 0,
                0, 1, 0, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0
            ]
        ),
        'k'
    );
}
