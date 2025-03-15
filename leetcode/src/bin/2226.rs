impl Solution {
    fn check(candies: &[i64], k: i64, mid: i64) -> bool {
        candies.iter().map(|&candy| candy / mid).sum::<i64>() >= k
    }

    pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
        let candies: Vec<i64> = candies.iter().map(|&x| x as i64).collect();

        if candies.iter().sum::<i64>() < k {
            return 0;
        }

        let mut l: i64 = 1;

        let sum = candies.iter().sum::<i64>();
        let avg = sum / k;
        let max_pile = candies.iter().max().unwrap_or(&0);
        let mut r = std::cmp::min(*max_pile, avg);

        while l <= r {
            let mid = l + (r - l) / 2;
            if Self::check(&candies, k, mid) {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }

        r as i32
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::maximum_candies(vec![5, 8, 6], 3), 5);
    assert_eq!(Solution::maximum_candies(vec![2, 5], 11), 0);
    assert_eq!(Solution::maximum_candies(vec![4, 7, 5], 16), 1);
    assert_eq!(Solution::maximum_candies(vec![4, 7, 5], 4), 3);
}
