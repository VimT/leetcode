//! 检查是否有合法括号字符串路径

/// dp[i][j][k] 表示走到 (i,j) 的时候，有k个左括号，能否返回true
pub fn has_valid_path(grid: Vec<Vec<char>>) -> bool {
    let m = grid.len();
    let n = grid[0].len();
    if grid[0][0] == ')' || grid[m - 1][n - 1] == '(' { return false; }
    let blen = ((m + n + 1) / 2).max(2);
    let mut dp = vec![vec![vec![false; blen]; n]; m];
    dp[m - 1][n - 1][1] = true;
    for i in (0..m).rev() {
        for j in (0..n).rev() {
            if i == m - 1 && j == n - 1 { continue; }
            for k in 0..blen as i32 {
                let nk = if grid[i][j] == '(' { k + 1 } else { k - 1 };
                if nk >= 0 && nk < blen as i32 {
                    dp[i][j][k as usize] = if i + 1 < m { dp[i + 1][j][nk as usize] } else { false } || if j + 1 < n { dp[i][j + 1][nk as usize] } else { false };
                }
            }
        }
    }
    dp[0][0][0]
}

pub fn has_valid_path_dfs(grid: Vec<Vec<char>>) -> bool {
    static DIR: [(usize, usize); 2] = [(1, 0), (0, 1)];
    fn dfs(grid: &Vec<Vec<char>>, i: usize, j: usize, left: usize, cache: &mut Vec<Vec<Vec<bool>>>) -> bool {
        if i == grid.len() - 1 && j == grid[0].len() - 1 {
            return left == 0;
        }
        if left >= cache[0][0].len() || cache[i][j][left] {
            return false;
        }
        for (di, dj) in DIR {
            let (ni, nj) = (i + di, j + dj);
            if ni < grid.len() && nj < grid[0].len() {
                if grid[ni][nj] == '(' {
                    if dfs(grid, ni, nj, left + 1, cache) {
                        return true;
                    }
                } else if left > 0 {
                    if dfs(grid, ni, nj, left - 1, cache) {
                        return true;
                    }
                }
            }
        }
        cache[i][j][left] = true;
        return false;
    }
    let m = grid.len();
    let n = grid[0].len();
    if grid[0][0] == ')' || grid[m - 1][n - 1] == '(' { return false; }
    let blen = ((m + n + 1) / 2).max(2);
    let mut cache = vec![vec![vec![false; blen]; n]; m];
    dfs(&grid, 0, 0, 1, &mut cache)
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<char>>) -> bool) {
        assert_eq!(func(vec![vec!['(', ')'], vec!['(', ')']]), false);
        assert_eq!(func(vec![vec!['(', ')']]), true);
        assert_eq!(func(vec![vec!['(', '(', '('], vec![')', '(', ')'], vec!['(', '(', ')'], vec!['(', '(', ')']]), true);
        assert_eq!(func(vec![vec![')', ')'], vec!['(', '(']]), false);
    }
    test(has_valid_path);
    test(has_valid_path_dfs);
}
