macro_rules! s {
    ($str:expr) => {
        String::from($str)
    };
}

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let n = words.len();
        let mut res = vec![];
        let mut i = 0;
        while i < n {
            let mut j = i;
            let mut curr = 0;
            while j < n && curr +words[j].len() <= max_width as usize {
                curr += words[j].len() + 1;
                j += 1;
            }
            let mut row = vec![];
            let mut curr_width = 0;
            for k in i..j {
                row.push(words[k].clone());
                curr_width += words[k].len();
            }
            let remaining = max_width as usize - curr_width;
            let mut line = String::new();
            if row.len() == 1 {
                line = row[0].clone();
                line.push_str(&" ".repeat(remaining));
            } else if j < n {
                let spaces = remaining / (row.len() - 1);
                let extra = remaining % (row.len() - 1);
                for k in 0..row.len() {
                    line.push_str(&row[k]);
                    if k == row.len() - 1 {
                        break;
                    }
                    if k < extra {
                        line.push_str(&" ".repeat(spaces + 1));
                    } else {
                        line.push_str(&" ".repeat(spaces));
                    }
                }
            } else if j == n {
                for k in 0..row.len() {
                    line.push_str(&row[k]);
                    if k < row.len() - 1 {
                        line.push_str(&" ");
                    }
                }
                line.push_str(&" ".repeat(remaining - row.len() + 1));
            }
            res.push(line);
            i = j;
        }
        res
    }
}

struct Solution;

fn main() {
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "1");
    }
    assert_eq!(
        Solution::full_justify(
            vec![
                s!("This"),
                s!("is"),
                s!("an"),
                s!("example"),
                s!("of"),
                s!("text"),
                s!("justification.")
            ],
            16
        ),
        vec![
            s!("This    is    an"),
            s!("example  of text"),
            s!("justification.  ")
        ]
    );
}
