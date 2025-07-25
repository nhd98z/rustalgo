impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
        let n = s.len();
        let mut sum = vec![0i64; n + 1];
        for e in shifts {
            if e[2] == 1 {
                sum[e[0] as usize] += 1;
                sum[e[1] as usize + 1] -= 1;
            } else {
                sum[e[0] as usize] -= 1;
                sum[e[1] as usize + 1] += 1;
            }
        }
        for i in 1..n {
            sum[i] += sum[i - 1];
        }
        let mut s = s.into_bytes();
        for i in 0..s.len() {
            let mut c = s[i] as i64 - 97;
            c = ((c + sum[i]) % 26 + 26) % 26;
            s[i] = c as u8 + 97;
        }
        String::from_utf8(s).unwrap()
    }

    pub fn shifting_letters_rusty(s: String, shifts: Vec<Vec<i32>>) -> String {
        let n = s.len();
        let mut diff = vec![0i64; n + 1];
        shifts.iter().for_each(|shift| {
            let (start, end, direction) = (shift[0] as usize, shift[1] as usize, shift[2]);
            let value = if direction == 1 { 1 } else { -1 };
            diff[start] += value;
            diff[end + 1] -= value;
        });

        for i in 1..n {
            diff[i] += diff[i - 1];
        }

        s.into_bytes()
            .iter()
            .enumerate()
            .map(|(i, &c)| {
                let shift = ((c as i64 - 97 + diff[i]) % 26 + 26) % 26;
                (shift as u8 + 97) as char
            })
            .collect()
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::shifting_letters("abc".to_string(), vec![vec![0, 1, 0], vec![1, 2, 1], vec![0, 2, 1]]),
        "ace".to_string()
    );

    assert_eq!(
        Solution::shifting_letters_rusty("abc".to_string(), vec![vec![0, 1, 0], vec![1, 2, 1], vec![0, 2, 1]]),
        "ace".to_string()
    );
}
