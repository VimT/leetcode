//! 黄金矿工

pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
    const DIR: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    fn dfs(grid: &Vec<Vec<i32>>, vis: &mut Vec<Vec<bool>>, cur: i32, pos: (usize, usize), result: &mut i32) {
        *result = (*result).max(cur);
        for (dx, dy) in DIR {
            let (nx, ny) = (pos.0 as i32 + dx, pos.1 as i32 + dy);
            if nx >= 0 && nx < grid.len() as i32 && ny >= 0 && ny < grid[0].len() as i32 {
                let (nx, ny) = (nx as usize, ny as usize);
                if grid[nx][ny] > 0 && !vis[nx][ny] {
                    vis[nx][ny] = true;
                    dfs(grid, vis, cur + grid[nx][ny], (nx, ny), result);
                    vis[nx][ny] = false;
                }
            }
        }
    }
    let m = grid.len();
    let n = grid[0].len();
    let mut result = 0;
    let mut vis = vec![vec![false; n]; m];
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] != 0 {
                vis[i][j] = true;
                dfs(&grid, &mut vis, grid[i][j], (i, j), &mut result);
                vis[i][j] = false;
            }
        }
    }
    result
}

fn main() {
    assert_eq!(get_maximum_gold(vec![vec![0, 6, 0], vec![5, 8, 7], vec![0, 9, 0]]), 24);
    assert_eq!(get_maximum_gold(vec![vec![1, 0, 7], vec![2, 0, 6], vec![3, 4, 5], vec![0, 3, 0], vec![9, 0, 20]]), 28);
}
