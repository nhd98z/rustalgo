impl Solution {
    pub fn max_free_time(event_time: i32, k: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let n = start_time.len();
        let mut spaces = vec![];
        spaces.push(start_time[0] - 0);
        for i in 1..n {
            spaces.push(start_time[i] - end_time[i - 1]);
        }
        spaces.push(event_time - end_time[n - 1]);
        let mut sum = vec![];
        sum.push(spaces[0]);
        for i in 1..spaces.len() {
            sum.push(sum[i - 1] + spaces[i]);
        }
        let mut res = 0;
        for i in k as usize..sum.len() {
            if i as i32 - k - 1 >= 0 {
                res = res.max(sum[i] - sum[i - k as usize - 1]);
            } else {
                res = res.max(sum[i]);
            }
        }
        res
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::max_free_time(5, 1, vec![1, 3], vec![2, 5]), 2);
    assert_eq!(Solution::max_free_time(21, 2, vec![18, 20], vec![20, 21]), 18);
}
