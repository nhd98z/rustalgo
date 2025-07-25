impl Solution {
    pub fn remove_duplicates(a: &mut Vec<i32>) -> i32 {
        let mut i = 2;
        while i < a.len() {
            if a[i] == a[i - 1] && a[i - 1] == a[i - 2] {
                a.remove(i);
            } else {
                i += 1;
            }
        }
        a.len() as i32
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::remove_duplicates(&mut vec![1, 1, 1, 2, 2, 3]), 5);
    assert_eq!(Solution::remove_duplicates(&mut vec![0, 0, 1, 1, 1, 1, 2, 3, 3]), 7);
}
