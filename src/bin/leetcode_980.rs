//! 不同路径 III

pub fn unique_paths_iii(mut grid: Vec<Vec<i32>>) -> i32 {
    static DIR: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    fn dfs(grid: &mut Vec<Vec<i32>>, x: usize, y: usize, left: i32, result: &mut i32) {
        if grid[x][y] == 2 {
            if left == 0 { *result += 1; }
            return;
        }
        grid[x][y] = 3;
        let m = grid.len();
        let n = grid[0].len();
        for (dx, dy) in DIR {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                let (nx, ny) = (nx as usize, ny as usize);
                if grid[nx][ny] & 1 == 0 {
                    dfs(grid, nx, ny, left - 1, result);
                }
            }
        }
        grid[x][y] = 0;
    }
    let m = grid.len();
    let n = grid[0].len();
    let mut start = (0, 0);
    let mut left = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 {
                start = (i, j);
            }
            if grid[i][j] != -1 {
                left += 1;
            }
        }
    }
    let mut result = 0;
    dfs(&mut grid, start.0, start.1, left - 1, &mut result);
    result
}

fn main() {
    assert_eq!(unique_paths_iii(vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 2, -1]]), 2);
    assert_eq!(unique_paths_iii(vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 2]]), 4);
    assert_eq!(unique_paths_iii(vec![vec![0, 1], vec![2, 0]]), 0);
}
