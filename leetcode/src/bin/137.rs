impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut ones = 0;
        let mut twos = 0;
        for num in nums {
            twos |= ones & num;
            ones ^= num;
            let common = ones & twos;
            twos &= !common;
            ones &= !common;
        }
        ones
    }
    pub fn single_number_version_2(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in 0..32 {
            let mut bit_sum = 0;
            for &num in &nums {
                if (num >> i) & 1 == 1 {
                    bit_sum += 1;
                }
            }
            if bit_sum % 3 != 0 {
                if i == 31 {
                    result -= 1 << 31; // Explicitly set as negative
                } else {
                    result |= 1 << i;
                }
            }
        }
        result
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::single_number(vec![2, 2, 3, 2]), 3);
    assert_eq!(Solution::single_number(vec![0, 1, 0, 1, 0, 1, 99]), 99);
    assert_eq!(Solution::single_number_version_2(vec![2, 2, 3, 2]), 3);
    assert_eq!(Solution::single_number_version_2(vec![0, 1, 0, 1, 0, 1, 99]), 99);
}
