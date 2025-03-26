impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let mut grid = grid.into_iter().flatten().collect::<Vec<i32>>();

        // Early validation: check if all elements have the same remainder when divided by x
        if grid.len() > 1 {
            let remainder = grid[0] % x;
            for &num in &grid {
                if num % x != remainder {
                    return -1;
                }
            }
        }

        // Find median using nth_element - O(n) operation instead of O(n log n) sort
        let n = grid.len();
        let median_idx = n / 2;
        grid.select_nth_unstable(median_idx);
        let median = grid[median_idx];

        // Calculate operations
        let mut cnt = 0;
        for item in grid {
            cnt += (item - median).abs() / x;
        }

        cnt
    }
}

struct Solution;

fn main() {
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "1");
    }
    assert_eq!(Solution::min_operations(vec![vec![2, 4], vec![6, 8]], 2), 4);
}
