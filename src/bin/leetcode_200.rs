//! 岛屿数量

use std::collections::VecDeque;

pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
    let rows = grid.len();
    if rows == 0 { return 0; }
    let cols = grid[0].len();
    let mut ans = 0;

    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == '1' {
                ans += 1;
                grid[i][j] = '0';
                let mut queue = VecDeque::new();
                queue.push_back((i, j));
                while !queue.is_empty() {
                    let (x, y) = queue.pop_front().unwrap();
                    if x != 0 && grid[x - 1][y] == '1' {
                        queue.push_back((x - 1, y));
                        grid[x - 1][y] = '0';
                    }
                    if x < rows - 1 && grid[x + 1][y] == '1' {
                        queue.push_back((x + 1, y));
                        grid[x + 1][y] = '0';
                    }
                    if y != 0 && grid[x][y - 1] == '1' {
                        queue.push_back((x, y - 1));
                        grid[x][y - 1] = '0';
                    }
                    if y < cols - 1 && grid[x][y + 1] == '1' {
                        queue.push_back((x, y + 1));
                        grid[x][y + 1] = '0';
                    }
                }
            }
        }
    }
    ans
}

fn main() {
    assert_eq!(num_islands(vec![vec!['1', '1', '0', '0', '0'], vec!['1', '1', '0', '0', '0'], vec!['0', '0', '1', '0', '0'], vec!['0', '0', '0', '1', '1']]), 3);
}
