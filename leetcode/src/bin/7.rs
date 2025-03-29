impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x;
        let mut res = 0;
        while x != 0 {
            if res > i32::MAX / 10 || res < i32::MIN / 10 {
                return 0;
            }
            res = res * 10 + x % 10;
            x /= 10;
        }
        res
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::reverse(123), 321);
    assert_eq!(Solution::reverse(-123), -321);
    assert_eq!(Solution::reverse(123456789), 987654321);
    assert_eq!(Solution::reverse(-123456789), -987654321);
}
