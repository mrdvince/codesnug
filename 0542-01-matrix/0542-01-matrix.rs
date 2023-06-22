use std::collections::VecDeque;

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        let rows = mat.len();
        let columns = mat[0].len();
        let mut dist: Vec<Vec<i32>> = vec![vec![i32::MAX; columns]; rows];
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

        // Initialize the queue with cells which contain 0s.
        for i in 0..rows {
            for j in 0..columns {
                if mat[i][j] == 0 {
                    dist[i][j] = 0;
                    queue.push_back((i, j));
                }
            }
        }

        // Process the queue.
        while let Some((row, col)) = queue.pop_front() {
            // Check the neighbors of the current cell.
            for dir in &directions {
                let new_row = (row as i32) + dir.0;
                let new_col = (col as i32) + dir.1;

                if new_row >= 0
                    && new_row < (rows as i32)
                    && new_col >= 0
                    && new_col < (columns as i32)
                {
                    let new_row = new_row as usize;
                    let new_col = new_col as usize;

                    // If the new distance is smaller, update the distance and add the cell to the queue.
                    if dist[row][col] + 1 < dist[new_row][new_col] {
                        dist[new_row][new_col] = dist[row][col] + 1;
                        queue.push_back((new_row, new_col));
                    }
                }
            }
        }

        dist
    }
}
