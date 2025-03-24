impl Solution {
    pub fn count_days(days: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        meetings.sort_by_key(|m| m[0]);

        let mut merged_meetings: Vec<Vec<i32>> = Vec::new();
        for meeting in meetings {
            if merged_meetings.is_empty() || meeting[0] > merged_meetings.last().unwrap()[1] {
                merged_meetings.push(meeting);
            } else {
                let last = merged_meetings.last_mut().unwrap();
                last[1] = last[1].max(meeting[1]);
            }
        }

        let meeting_days_count: i32 = merged_meetings.iter().map(|m| m[1] - m[0] + 1).sum();

        days - meeting_days_count
    }
}

struct Solution;

fn main() {
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    assert_eq!(
        Solution::count_days(10, vec![vec![5, 7], vec![1, 3], vec![9, 10]]),
        2
    );

    assert_eq!(
        Solution::count_days(15, vec![vec![1, 5], vec![4, 8], vec![10, 14]]),
        2
    );

    assert_eq!(
        Solution::count_days(12, vec![vec![1, 3], vec![4, 6], vec![7, 9], vec![10, 12]]),
        0
    );

    assert_eq!(
        Solution::count_days(
            14,
            vec![
                vec![6, 11],
                vec![7, 13],
                vec![8, 9],
                vec![5, 8],
                vec![3, 13],
                vec![11, 13],
                vec![1, 3],
                vec![5, 10],
                vec![8, 13],
                vec![3, 9]
            ]
        ),
        1
    );

    assert_eq!(
        Solution::count_days(
            50,
            vec![
                vec![22, 31],
                vec![7, 42],
                vec![30, 46],
                vec![9, 33],
                vec![9, 18],
                vec![23, 39],
                vec![4, 8],
                vec![18, 19]
            ]
        ),
        7
    );
}
