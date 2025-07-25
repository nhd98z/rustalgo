use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};

impl Solution {
    pub fn min_stickers_stupid(stickers: Vec<String>, target: String) -> i32 {
        // Preprocess stickers into frequency maps
        let sticker_maps: Vec<[i32; 26]> = stickers
            .iter()
            .map(|sticker| {
                let mut count = [0; 26];
                for c in sticker.chars() {
                    count[(c as u8 - b'a') as usize] += 1;
                }
                count
            })
            .collect();

        // Memoization map
        let mut memo = HashMap::new();
        memo.insert("".to_string(), 0); // Base case: empty target requires 0 stickers

        // Call the recursive helper function
        let res = Self::helper(&mut memo, &sticker_maps, &target);

        if res == i32::MAX { -1 } else { res }
    }

    fn helper(memo: &mut HashMap<String, i32>, sticker_maps: &[[i32; 26]], target: &str) -> i32 {
        if let Some(&res) = memo.get(target) {
            return res; // Return cached result if available
        }

        let mut target_count = [0; 26];
        for c in target.chars() {
            target_count[(c as u8 - b'a') as usize] += 1;
        }

        let mut ans = i32::MAX;

        // Try using each sticker
        for sticker in sticker_maps {
            // Optimization: Skip stickers that don't contribute to the first letter
            if sticker[(target.chars().next().unwrap() as u8 - b'a') as usize] == 0 {
                continue;
            }

            // Apply this sticker to the target
            let mut new_target = String::new();
            for i in 0..26 {
                if target_count[i] > 0 {
                    let remaining = target_count[i] - sticker[i];
                    if remaining > 0 {
                        new_target.push_str(
                            &std::iter::repeat((b'a' + i as u8) as char)
                                .take(remaining as usize)
                                .collect::<String>(),
                        );
                    }
                }
            }

            // Recursive call
            let tmp = Self::helper(memo, sticker_maps, &new_target);
            if tmp != i32::MAX {
                ans = ans.min(1 + tmp);
            }
        }

        // Cache the result for the current target
        memo.insert(target.to_string(), ans);
        ans
    }
}

impl Solution {
    pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
        // Optimization 1: Maintain frequency only for characters present in target

        // Step 1: Map characters in target to indices
        let mut index = [-1; 26];
        let mut chars = Vec::new();
        for c in target.chars() {
            let idx = (c as u8 - b'a') as usize;
            if index[idx] == -1 {
                index[idx] = chars.len() as i32;
                chars.push(c);
            }
        }
        let n_chars = chars.len();

        // Step 2: Build frequency count for target
        let mut target_count = vec![0u8; n_chars];
        for c in target.chars() {
            let idx = index[(c as u8 - b'a') as usize] as usize;
            target_count[idx] += 1;
        }

        // Step 3: Build frequency count for stickers
        let mut stickers_count = vec![vec![0u8; n_chars]; stickers.len()];
        for (i, sticker) in stickers.iter().enumerate() {
            for c in sticker.chars() {
                let idx = index[(c as u8 - b'a') as usize];
                if idx >= 0 {
                    stickers_count[i][idx as usize] += 1;
                }
            }
        }

        // Optimization 2: Remove dominated stickers
        let mut start = 0;
        for i in 0..stickers.len() {
            for j in start..stickers.len() {
                if i != j {
                    let mut k = 0;
                    while k < n_chars && stickers_count[i][k] <= stickers_count[j][k] {
                        k += 1;
                    }
                    if k == n_chars {
                        stickers_count.swap(i, start);
                        start += 1;
                        break;
                    }
                }
            }
        }

        // BFS initialization
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back(target_count.clone());
        let mut steps = 0;

        // BFS traversal
        while !queue.is_empty() {
            steps += 1;
            for _ in 0..queue.len() {
                let freq = queue.pop_front().unwrap();
                let state = State(freq.clone());
                if visited.insert(state) {
                    let first = match freq.iter().position(|&f| f > 0) {
                        Some(pos) => pos,
                        None => return steps - 1,
                    };
                    for i in start..stickers.len() {
                        if stickers_count[i][first] > 0 {
                            let mut next = freq.clone();
                            for j in 0..n_chars {
                                next[j] = next[j].saturating_sub(stickers_count[i][j]);
                            }
                            if next.iter().all(|&f| f == 0) {
                                return steps;
                            }
                            queue.push_back(next);
                        }
                    }
                }
            }
        }
        -1
    }
}

#[derive(Clone)]
struct State(Vec<u8>);

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for State {}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::min_stickers_stupid(
            vec!["with".to_string(), "example".to_string(), "science".to_string()],
            "thehat".to_string()
        ),
        3
    );
    assert_eq!(
        Solution::min_stickers(
            vec!["with".to_string(), "example".to_string(), "science".to_string()],
            "thehat".to_string()
        ),
        3
    );
}
