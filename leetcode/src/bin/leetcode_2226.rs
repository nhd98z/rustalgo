impl Solution {
    fn check(candies: &[i64], k: i64, mid: i64) -> bool {
        let mut cnt = 0;
        for &candy in candies {
            cnt += candy / mid;
        }
        cnt >= k
    }
    pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
        let candies: Vec<i64> = candies.iter().map(|x| *x as i64).collect();
        let mut l: i64 = 0;
        let mut r: i64 = candies.iter().sum::<i64>() / k + 1;
        while l <= r {
            let mid = l + (r - l) / 2;
            if mid != 0 && Self::check(&candies, k, mid) {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
        if l == 0 { 0 } else { (l as i32) - 1 }
    }
}

struct Solution;

fn main() {
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "1");
    }
    assert_eq!(Solution::maximum_candies(vec![5, 8, 6], 3), 5);
    assert_eq!(Solution::maximum_candies(vec![2, 5], 11), 0);
    assert_eq!(Solution::maximum_candies(vec![4, 7, 5], 16), 1);
    assert_eq!(Solution::maximum_candies(vec![4, 7, 5], 4), 3);
}
