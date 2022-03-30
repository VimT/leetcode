//! 摘樱桃


pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
    fn dfs(grid: &Vec<Vec<i32>>, x1: usize, y1: usize, x2: usize, cache: &mut Vec<Vec<Vec<i32>>>) -> i32 {
        let y2 = x1 + y1 - x2;
        let len = grid.len();
        if x1 == len || y1 == len || x2 == len || y2 == len || grid[x1][y1] == -1 || grid[x2][y2] == -1 {
            return -1;
        }
        if cache[x1][y1][x2] != i32::MAX {
            return cache[x1][y1][x2];
        }
        if x1 == len - 1 && y1 == len - 1 {
            return grid[x1][y1];
        }
        let mut result = dfs(grid, x1 + 1, y1, x2, cache)
            .max(dfs(grid, x1, y1 + 1, x2, cache))
            .max(dfs(grid, x1 + 1, y1, x2 + 1, cache))
            .max(dfs(grid, x1, y1 + 1, x2 + 1, cache));
        if result >= 0 {
            result += grid[x1][y1];
            if x1 != x2 {
                result += grid[x2][y2];
            }
        }
        cache[x1][y1][x2] = result;
        return result;
    }
    let len = grid.len();
    dfs(&grid, 0, 0, 0, &mut vec![vec![vec![i32::MAX; len]; len]; len]).max(0)
}

pub fn cherry_pickup_dp(grid: Vec<Vec<i32>>) -> i32 {
    let len = grid.len();
    let mut dp = vec![vec![vec![i32::MIN; len + 1]; len + 1]; len + 1];
    dp[1][0][0] = 0;
    dp[0][1][0] = 0;
    for x1 in 1..=len {
        for y1 in 1..=len {
            for x2 in (x1 as i32 + y1 as i32 - len as i32).max(1) as usize..=(x1 + y1 - 1).min(len) {
                let y2 = x1 + y1 - x2;
                if grid[x1 - 1][y1 - 1] != -1 && grid[x2 - 1][y2 - 1] != -1 {
                    dp[x1][y1][x2] = dp[x1 - 1][y1][x2 - 1]
                        .max(dp[x1 - 1][y1][x2])
                        .max(dp[x1][y1 - 1][x2])
                        .max(dp[x1][y1 - 1][x2 - 1])
                        + grid[x1 - 1][y1 - 1]
                        + if x1 != x2 { grid[x2 - 1][y2 - 1] } else { 0 };
                }
            }
        }
    }
    dp[len][len][len].max(0)
}

fn main() {
    assert_eq!(cherry_pickup_dp(vec![vec![1, 1, -1], vec![1, -1, 1], vec![-1, 1, 1]]), 0);
    assert_eq!(cherry_pickup_dp(vec![vec![1]]), 1);
    assert_eq!(cherry_pickup_dp(vec![vec![0, 1, -1], vec![1, 0, -1], vec![1, 1, 1]]), 5);
}
