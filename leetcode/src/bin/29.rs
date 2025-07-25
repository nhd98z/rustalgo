impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if divisor == 1 {
            return dividend;
        }
        if dividend == i32::MIN && divisor == -1 {
            return i32::MAX;
        }
        if divisor == i32::MIN {
            return if dividend == i32::MIN { 1 } else { 0 };
        }
        let negative = (dividend < 0) ^ (divisor < 0);
        let mut quotient = 0;
        if dividend == i32::MIN && dividend % divisor == 0 {
            quotient += 1;
        }
        let mut dividend = if dividend == i32::MIN { i32::MAX } else { dividend.abs() };
        let divisor = if divisor == i32::MIN { i32::MAX } else { divisor.abs() };
        loop {
            let mut exp = 0;
            while 0 < (divisor << exp) && (divisor << exp) <= dividend {
                exp += 1;
            }
            if exp == 0 {
                break;
            }
            quotient += 1 << (exp - 1);
            dividend -= divisor << (exp - 1);
        }
        if negative { -quotient } else { quotient }
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::divide(10, 3), 3);
    assert_eq!(Solution::divide(7, -3), -2);
    assert_eq!(Solution::divide(i32::MIN, -1), i32::MAX);
    assert_eq!(Solution::divide(i32::MAX, -1), -i32::MAX);
    assert_eq!(Solution::divide(1, 1), 1);
    assert_eq!(Solution::divide(2, 1), 2);
    assert_eq!(Solution::divide(2, 2), 1);
    assert_eq!(Solution::divide(i32::MAX, 1), i32::MAX);
    assert_eq!(Solution::divide(i32::MIN, 1), i32::MIN);
    assert_eq!(Solution::divide(i32::MIN, 2), -1073741824);
    assert_eq!(Solution::divide(i32::MAX, i32::MIN), 0);
    assert_eq!(Solution::divide(i32::MIN, 122481295), -17);
    assert_eq!(Solution::divide(i32::MIN, i32::MIN), 1);
}
