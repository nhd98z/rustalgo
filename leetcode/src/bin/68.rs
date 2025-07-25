macro_rules! s {
    ($str:expr) => {
        String::from($str)
    };
}

impl Solution {
    /// Formats the input words into justified text with the specified maximum width.
    ///
    /// # Arguments
    /// * `words` - A vector of strings to be justified
    /// * `max_width` - The maximum width of each line
    ///
    /// # Returns
    /// A vector of strings where each string is a justified line
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let n = words.len();
        let mut res = vec![]; // Result vector to store justified lines
        let mut i = 0; // Current starting word index

        // Process all words
        while i < n {
            // Step 1: Determine which words fit on the current line
            let mut j = i; // End word index (exclusive)
            let mut curr_len = 0; // Current line length including spaces

            // Add words to the current line as long as they fit within max_width
            while j < n && curr_len + words[j].len() + (if curr_len > 0 { 1 } else { 0 }) <= max_width as usize {
                if curr_len > 0 {
                    curr_len += 1; // Add space between words
                }
                curr_len += words[j].len();
                j += 1;
            }

            // Step 2: Collect words for the current line
            let mut row = vec![]; // Words in the current line
            let mut curr_width = 0; // Total width of words (without spaces)

            for k in i..j {
                row.push(words[k].clone());
                curr_width += words[k].len();
            }

            // Calculate remaining space to distribute
            let remaining = max_width as usize - curr_width;
            let mut line = String::new();

            // Step 3: Format the line based on justification rules
            if row.len() == 1 {
                // Case 1: Single word line - left justify with trailing spaces
                line = row[0].clone();
                line.push_str(&" ".repeat(remaining));
            } else if j < n {
                // Case 2: Middle line with multiple words - distribute spaces evenly
                let spaces = remaining / (row.len() - 1); // Minimum spaces between words
                let extra = remaining % (row.len() - 1); // Extra spaces to distribute from left

                for k in 0..row.len() {
                    line.push_str(&row[k]);

                    // Don't add spaces after the last word
                    if k == row.len() - 1 {
                        break;
                    }

                    // Add extra space to the leftmost slots if needed
                    if k < extra {
                        line.push_str(&" ".repeat(spaces + 1));
                    } else {
                        line.push_str(&" ".repeat(spaces));
                    }
                }
            } else if j == n {
                // Case 3: Last line - left justify with single spaces and trailing spaces
                for k in 0..row.len() {
                    line.push_str(&row[k]);

                    // Add a single space between words (not after the last word)
                    if k < row.len() - 1 {
                        line.push_str(&" ");
                    }
                }

                // Add trailing spaces to reach max_width
                line.push_str(&" ".repeat(remaining - row.len() + 1));
            }

            res.push(line);
            i = j; // Move to the next set of words
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
        vec![s!("This    is    an"), s!("example  of text"), s!("justification.  ")]
    );
}
