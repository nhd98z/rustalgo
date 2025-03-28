use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

impl Solution {
    pub fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let m = grid.len();
        let n = grid[0].len();

        let mut result = vec![0; queries.len()];

        let mut query_indices: Vec<(i32, usize)> = queries.iter().enumerate().map(|(i, v)| (*v, i)).collect();
        query_indices.sort_unstable();

        let mut heap = BinaryHeap::new();
        heap.push(Reverse((grid[0][0], 0, 0)));
        let mut visited = HashSet::new();
        visited.insert((0, 0));

        let direction = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut count = 0;

        for (query_value, query_idx) in query_indices {
            while let Some(Reverse((cell_value, row, col))) = heap.pop() {
                if cell_value >= query_value {
                    heap.push(Reverse((cell_value, row, col)));
                    break;
                }

                count += 1;

                for (dr, dc) in &direction {
                    let new_row = row as i32 + dr;
                    let new_col = col as i32 + dc;

                    if new_row >= 0 && new_row < m as i32 && new_col >= 0 && new_col < n as i32 {
                        let new_row = new_row as usize;
                        let new_col = new_col as usize;

                        if !visited.contains(&(new_row, new_col)) {
                            visited.insert((new_row, new_col));
                            heap.push(Reverse((grid[new_row][new_col], new_row, new_col)));
                        }
                    }
                }
            }

            result[query_idx] = count;
        }

        result
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::max_points(
            vec![vec![1, 2, 3], vec![2, 5, 7], vec![3, 5, 1]],
            vec![5, 6, 2]
        ),
        vec![5, 8, 1]
    );
    assert_eq!(
        Solution::max_points(vec![vec![5, 2, 1], vec![1, 1, 2]], vec![3]),
        vec![0]
    );
}
