impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = strs[0].clone();
        for s in strs.iter().skip(1) {
            while !s.starts_with(&prefix) {
                prefix = prefix[..prefix.len() - 1].to_string();
            }
        }
        prefix
    }
}

macro_rules! s {
    ($str:expr) => {
        String::from($str)
    };
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::longest_common_prefix(vec![s!("flower"), s!("flow"), s!("flight")]),
        s!("fl")
    );
    assert_eq!(
        Solution::longest_common_prefix(vec![s!("dog"), s!("racecar"), s!("car")]),
        s!("")
    );
}
