impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        // Use Cantor's diagonalization: for each string's i-th position,
        // pick the opposite bit for our result's i-th position
        nums.iter()
            .enumerate()
            .map(|(i, num)| {
                // Get the i-th character and flip it
                if num.chars().nth(i).unwrap() == '0' { '1' } else { '0' }
            })
            .collect()
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::find_different_binary_string(vec!["01".to_string(), "10".to_string()]),
        "11"
    );
}
