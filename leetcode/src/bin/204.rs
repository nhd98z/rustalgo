impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n <= 2 {
            return 0;
        }

        let n = n as usize;
        let mut is_prime = vec![true; n];
        // 0 and 1 are not prime.
        is_prime[0] = false;
        is_prime[1] = false;

        let sqrt = (n as f64).sqrt() as usize;
        for i in 2..=sqrt {
            if is_prime[i] {
                for j in (i * i..n).step_by(i) {
                    is_prime[j] = false;
                }
            }
        }

        is_prime.iter().filter(|&&x| x).count() as i32
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::count_primes(10), 4);
}
