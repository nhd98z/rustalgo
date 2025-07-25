impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates.clone();
        candidates.sort();
        let mut res: Vec<Vec<i32>> = vec![];
        let mut path: Vec<i32> = vec![];
        Solution::backtrack(&mut res, &candidates, target, 0, &mut path);
        res
    }

    fn backtrack(res: &mut Vec<Vec<i32>>, candidates: &Vec<i32>, remaining: i32, index: usize, path: &mut Vec<i32>) {
        if remaining == 0 {
            res.push(path.clone());
            return;
        }
        for i in index..candidates.len() {
            if candidates[i] > remaining {
                return;
            }
            path.push(candidates[i]);
            Solution::backtrack(res, candidates, remaining - candidates[i], i, path);
            path.pop();
        }
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::combination_sum(vec![2, 3, 6, 7], 7),
        vec![vec![2, 2, 3], vec![7]]
    );
    assert_eq!(
        Solution::combination_sum(vec![2, 3, 5], 8),
        vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
    );
    assert_eq!(Solution::combination_sum(vec![2], 1), vec![] as Vec<Vec<i32>>);
}
