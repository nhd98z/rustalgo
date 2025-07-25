impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        // O(n^2)
        let s: Vec<u8> = s.bytes().into_iter().collect();
        let n = s.len() as i32;
        let mut res: String = (s[0] as char).to_string();
        for i in 0..n {
            let mut j = i;
            let mut k = i;
            while j >= 0 && k < n && s[j as usize] == s[k as usize] {
                if k - j + 1 > res.len() as i32 {
                    res = String::from_utf8(s[j as usize..=k as usize].to_vec()).unwrap();
                }
                j -= 1;
                k += 1;
            }
            let mut j = i;
            let mut k = i + 1;
            while j >= 0 && k < n && s[j as usize] == s[k as usize] {
                if k - j + 1 > res.len() as i32 {
                    res = String::from_utf8(s[j as usize..=k as usize].to_vec()).unwrap();
                }
                j -= 1;
                k += 1;
            }
        }
        res
    }

    #[allow(dead_code, unused_variables)]
    pub fn longest_palindrome_manacher(s: String) -> String {
        todo!()
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::longest_palindrome("babad".to_string()), "bab".to_string());
    assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb".to_string());
    assert_eq!(Solution::longest_palindrome("a".to_string()), "a".to_string());
    assert_eq!(Solution::longest_palindrome("aa".to_string()), "aa".to_string());
    assert_eq!(Solution::longest_palindrome("aab".to_string()), "aa".to_string());
    assert_eq!(Solution::longest_palindrome("aba".to_string()), "aba".to_string());
}
