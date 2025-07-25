impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
        let mut s = s.chars().collect::<Vec<char>>();
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j {
            if vowels.contains(&s[i]) {
                if vowels.contains(&s[j]) {
                    s.swap(i, j);
                    i += 1;
                    j -= 1;
                } else {
                    j -= 1;
                }
            } else {
                i += 1;
            }
        }
        s.into_iter().collect()
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::reverse_vowels("hello".to_string()), "holle");
    assert_eq!(Solution::reverse_vowels("leetcode".to_string()), "leotcede");
}
