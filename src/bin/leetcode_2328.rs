//! 网格图中递增路径的数目


use std::collections::VecDeque;

const MOD: i32 = 1e9 as i32 + 7;
static DIR: [(i32, i32); 4] = [(-1, 0), (0, 1), (0, -1), (1, 0)];

/// 拓扑排序
pub fn count_paths(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();

    let mut indegree = vec![vec![0; n]; m];
    let mut q = VecDeque::new();
    for i in 0..m {
        for j in 0..n {
            for (dx, dy) in DIR {
                let (nx, ny) = (i as i32 + dx, j as i32 + dy);
                if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                    if grid[nx as usize][ny as usize] < grid[i][j] {
                        indegree[i][j] += 1;
                    }
                }
            }
            if indegree[i][j] == 0 {
                q.push_back((i, j));
            }
        }
    }
    let mut dp = vec![vec![1; n]; m];
    while !q.is_empty() {
        let (x, y) = q.pop_front().unwrap();
        for (dx, dy) in DIR {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                let (nx, ny) = (nx as usize, ny as usize);
                if grid[nx][ny] > grid[x][y] {
                    dp[nx][ny] = (dp[nx][ny] + dp[x][y]) % MOD;
                    indegree[nx][ny] -= 1;
                    if indegree[nx][ny] == 0 {
                        q.push_back((nx, ny));
                    }
                }
            }
        }
    }
    let mut result = 0;
    for i in 0..m {
        for j in 0..n {
            result = (result + dp[i][j]) % MOD;
        }
    }
    result
}

/// 记忆化搜索
pub fn count_paths2(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();

    fn dfs(grid: &Vec<Vec<i32>>, x: usize, y: usize, cache: &mut Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut result = 1;
        if cache[x][y] != -1 {
            return cache[x][y];
        }
        for (dx, dy) in DIR {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 && grid[nx as usize][ny as usize] > grid[x][y] {
                result = (result + dfs(grid, nx as usize, ny as usize, cache)) % MOD;
            }
        }
        cache[x][y] = result;
        return result;
    }
    let mut result = 0;
    let mut cache = vec![vec![-1; n]; m];
    for i in 0..m {
        for j in 0..n {
            result = (result + dfs(&grid, i, j, &mut cache)) % MOD;
        }
    }
    result
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![1, 1], vec![3, 4]]), 8);
        assert_eq!(func(vec![vec![1], vec![2]]), 3);
    }
    test(count_paths);
    test(count_paths2);
}
