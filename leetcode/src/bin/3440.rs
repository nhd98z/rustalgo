impl Solution {
    pub fn max_free_time(event_time: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let mut res = 0;
        let n = start_time.len();
        let mut spaces = vec![];
        spaces.push(start_time[0]);
        for i in 1..n {
            spaces.push(start_time[i] - end_time[i - 1]);
        }
        spaces.push(event_time - end_time[n - 1]);
        // dbg!(&spaces);
        let mut left_max = vec![0; n + 1];
        left_max[0] = spaces[0];
        for i in 1..n + 1 {
            left_max[i] = left_max[i - 1].max(spaces[i]);
        }
        // dbg!(&left_max);
        let mut right_max = vec![0; n + 1];
        right_max[n] = spaces[n];
        for i in (0..n).rev() {
            right_max[i] = right_max[i + 1].max(spaces[i]);
        }
        // dbg!(&right_max);
        for i in 0..n {
            let dur = end_time[i] - start_time[i];
            res = res.max(spaces[i] + spaces[i + 1]);
            let can_move_left = i > 0 && dur <= left_max[i - 1];
            let can_move_right = i + 2 <= n && dur <= right_max[i + 2];
            if can_move_left || can_move_right {
                res = res.max(spaces[i] + spaces[i + 1] + dur);
            }
        }
        res
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::max_free_time(99, vec![3, 16, 97], vec![12, 66, 98]), 35);
    assert_eq!(Solution::max_free_time(5, vec![1, 3], vec![2, 5]), 2);
    assert_eq!(Solution::max_free_time(10, vec![0, 3, 7, 9], vec![1, 4, 8, 10]), 6);
    assert_eq!(Solution::max_free_time(5, vec![0, 1, 2, 3, 4], vec![1, 2, 3, 4, 5]), 0);
}
