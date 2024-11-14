impl Solution {
    pub fn min_end(n: i32, x: i32) -> i64 {
        const MAX_BIT: usize = 64;

        let mut x = x;
        let mut bits = Vec::new();
        while x > 0 {
            bits.push((x % 2) as i64);
            x /= 2;
        }
        bits.reverse();
        bits.iter().fold(0, |acc, &b| acc * 10 + b);
        while bits.len() < MAX_BIT {
            bits.insert(0, 0);
        }
        bits.iter().fold(0, |acc, &b| acc * 10 + b);

        // dbg!(&bits);

        let mut n = n - 1;
        let mut n_bits = Vec::new();
        while n > 0 {
            n_bits.push((n % 2) as i64);
            n /= 2;
        }
        n_bits.reverse();
        while n_bits.len() < MAX_BIT {
            n_bits.insert(0, 0);
        }
        n_bits.iter().fold(0, |acc, &b| acc * 10 + b);

        let mut j = n_bits.len() as i32 - 1;
        for i in (0..bits.len()).rev() {
            if bits[i] == 0 {
                bits[i] = n_bits[j as usize];
                j -= 1;
                if j < 0 {
                    break;
                }
            }
        }

        // dbg!(&n_bits);

        bits.iter().fold(0, |acc, &b| acc * 2 + b)
    }
}

struct Solution;

fn main() {
    dbg!(Solution::min_end(2, 7));
}
