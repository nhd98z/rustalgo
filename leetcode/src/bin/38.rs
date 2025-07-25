impl Solution {
    fn count_and_say_once(s: String) -> String {
        let mut result = String::new();
        let mut count = 1;
        let mut chars = s.chars();
        let mut current_char = chars.next().unwrap();

        while let Some(next_char) = chars.next() {
            if next_char == current_char {
                count += 1;
            } else {
                result.push_str(&count.to_string());
                result.push(current_char);
                current_char = next_char;
                count = 1;
            }
        }
        result.push_str(&count.to_string());
        result.push(current_char);
        result
    }
    pub fn count_and_say(n: i32) -> String {
        // 1
        // 11
        // 21
        // 1211
        // 111221
        // 312211

        let mut result = String::from("1");
        for _ in 1..n {
            result = Self::count_and_say_once(result);
        }
        result
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::count_and_say(1), "1");
    assert_eq!(Solution::count_and_say(2), "11");
    assert_eq!(Solution::count_and_say(3), "21");
    assert_eq!(Solution::count_and_say(4), "1211");
    assert_eq!(Solution::count_and_say(5), "111221");
    assert_eq!(Solution::count_and_say(6), "312211");
    assert_eq!(Solution::count_and_say(7), "13112221");
    assert_eq!(Solution::count_and_say(8), "1113213211");
}
