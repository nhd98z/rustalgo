impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut left = 0;
        let mut right = n - 1;
        while left < right {
            let mid = left + (right - left) / 2;
            dbg!(left, right, mid);
            if arr[mid] < arr[mid + 1] {
                left = mid + 1;
            } else {
                right = mid
            }
        }
        left as i32
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 1, 0]), 1);
    assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 2, 1, 0]), 1);
    assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 10, 5, 2]), 1);
    assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 9, 10, 5, 2]), 2);
    assert_eq!(Solution::peak_index_in_mountain_array(vec![3, 4, 5, 1]), 2);
}
