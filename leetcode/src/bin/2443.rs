impl Solution {
    pub fn sum_of_number_and_reverse(num: i32) -> bool {
        let mut i = 0;
        while i <= num {
            let mut rev = 0;
            let mut n = i;
            while n > 0 {
                rev = rev * 10 + n % 10;
                n /= 10;
            }
            if i + rev == num {
                return true;
            }
            i += 1;
        }
        false

    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::sum_of_number_and_reverse(443), true);
}
