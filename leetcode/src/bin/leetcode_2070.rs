impl Solution {
    pub fn maximum_beauty(items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut items = items;
        items.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut prefix_max = vec![0; items.len()];
        for i in 0..items.len() {
            prefix_max[i] = items[i][1];
            if i > 0 {
                prefix_max[i] = prefix_max[i].max(prefix_max[i - 1]);
            }
        }
        let mut res = vec![0; queries.len()];
        for i in 0..queries.len() {
            let mut left = 0;
            let mut right = items.len() - 1;
            while left <= right {
                let mid = left + (right - left) / 2;
                if items[mid][0] <= queries[i] {
                    left = mid + 1;
                } else {
                    if mid == 0 {
                        break;
                    }
                    right = mid - 1;
                }
            }
            if left == 0 {
                res[i] = 0;
            } else {
                res[i] = prefix_max[left - 1];
            }
        }
        
        res
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::maximum_beauty(
            vec![vec![1, 2], vec![3, 2], vec![2, 4], vec![5, 6], vec![3, 5]],
            vec![1, 2, 3, 4, 5, 6]
        ),
        vec![2, 4, 5, 5, 6, 6]
    );
    assert_eq!(
        Solution::maximum_beauty(vec![vec![10, 1000]], vec![5]),
        vec![0]
    );
}
