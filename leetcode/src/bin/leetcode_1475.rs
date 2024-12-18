impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let n = prices.len();
        let mut res = prices.clone();
        let mut stack = vec![];
        for i in (0..n).rev() {
            while let Some(&top) = stack.last() {
                if top <= prices[i] {
                    break;
                }
                stack.pop();
            }
            if let Some(&top) = stack.last() {
                res[i] -= top;
            }
            stack.push(prices[i]);
        }
        res
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::final_prices(vec![8, 4, 6, 2, 3]),
        vec![4, 2, 4, 2, 3]
    );
}
