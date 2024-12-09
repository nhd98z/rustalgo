impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut res = 0;
        let mut ss = String::new();
        for c in s.chars() {
            if ss.contains(c) {
                while ss.chars().nth(0).unwrap_or(' ') != c {
                    ss.remove(0);
                }
                ss.remove(0);
                ss.push(c);
            } else {
                ss.push(c);
                res = usize::max(res, ss.len());
            }
        }
        res as i32
    }
}

struct Solution;

fn main() {
    dbg!(Solution::length_of_longest_substring(
        "abcabcbb".to_string()
    ));
    dbg!(Solution::length_of_longest_substring("bbbbb".to_string()));
    dbg!(Solution::length_of_longest_substring("pwwkew".to_string()));
    dbg!(Solution::length_of_longest_substring("aab".to_string()));
    dbg!(Solution::length_of_longest_substring("dvdf".to_string()));
    dbg!(Solution::length_of_longest_substring("tmmzuxt".to_string()));
}
