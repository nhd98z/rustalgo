impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        // Early return if odd length
        if nums.len() % 2 != 0 {
            return false;
        }

        // Use a fixed-size array for counting (assuming constraints)
        let mut counts = [0_u8; 501];

        // Count occurrences with bitwise operations
        for num in nums {
            counts[num as usize] ^= 1; // Toggle between 0 and 1
        }

        // Check if all counts are even (all should be 0)
        for &count in &counts {
            if count != 0 {
                return false;
            }
        }

        true
    }
}

struct Solution;

fn main() {
    unsafe { std::env::set_var("RUST_BACKTRACE", "1"); }
    assert_eq!(Solution::divide_array(vec![3, 2, 3, 2, 2, 2]), true);
}
