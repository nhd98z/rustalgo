#![allow(dead_code)]

impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut sum = vec![0; words.len()];
        let mut cnt = 0;
        let vowels = |c: u8| matches!(c, b'a' | b'e' | b'i' | b'o' | b'u');
        let words: Vec<&[u8]> = words.iter().map(|s| s.as_bytes()).collect();

        for (i, word) in words.iter().enumerate() {
            if vowels(word[0]) && vowels(word[word.len() - 1]) {
                cnt += 1;
            }
            sum[i] = cnt;
        }

        queries
            .iter()
            .map(|q| {
                if q[0] == 0 {
                    sum[q[1] as usize]
                } else {
                    sum[q[1] as usize] - sum[(q[0] - 1) as usize]
                }
            })
            .collect()
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::vowel_strings(
            vec![
                "aba".to_string(),
                "bcb".to_string(),
                "ece".to_string(),
                "aa".to_string(),
                "e".to_string()
            ],
            vec![vec![0, 2], vec![1, 4], vec![1, 1]]
        ),
        [2, 3, 0]
    );
}
