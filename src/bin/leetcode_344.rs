impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let n = s.len();
        for i in 0..n / 2 {
            s.swap(i, n - i - 1);
        }
    }
}

struct Solution;

fn main() {
    let mut s = vec!['h', 'e', 'l', 'l', 'o'];
    Solution::reverse_string(&mut s);
    
    assert_eq!(s, vec!['o', 'l', 'l', 'e', 'h']);
}
