//! 矩阵中的最长递增路径


/// 可以把 相邻元素 i<j, 转换成i->j， 则这就是一个有向图，结果是找图的最长路径， dfs
pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
    fn dfs(matrix: &Vec<Vec<i32>>, x: i32, y: i32, cache: &mut Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let rows = matrix.len() as i32;
        let cols = matrix[0].len() as i32;
        if cache[x as usize][y as usize] > 0 {
            return cache[x as usize][y as usize];
        }
        for (i, j) in vec![(-1, 0), (0, -1), (1, 0), (0, 1)] {
            if x + i >= 0 && x + i < rows && y + j >= 0 && y + j < cols && matrix[(x + i) as usize][(y + j) as usize] > matrix[x as usize][y as usize] {
                ans = dfs(matrix, x + i, y + j, cache).max(ans);
            }
        }
        ans += 1;
        cache[x as usize][y as usize] = ans;
        ans
    }
    let rows = matrix.len();
    if rows == 0 { return 0; }
    let cols = matrix[0].len();
    if cols == 0 { return 0; }
    let mut cache = vec![vec![0; cols]; rows];
    let mut ans = 0;
    for i in 0..rows {
        for j in 0..cols {
            ans = ans.max(dfs(&matrix, i as i32, j as i32, &mut cache));
        }
    }
    ans
}

/// dp[i][j] 表示从(i,j)开始的最长递增路径
/// dp[i][j] = max( dp[x,y] ) + 1  matrix[x][y] > matrix[i][j]
/// 但是没有依赖列表，也就是说，不知道要从哪个点开始遍历
/// 通过剥洋葱法，遍历dp[i][j]，把所有叶子节点都去掉之后，产生新的叶子节点，重复，直到找到根节点
pub fn longest_increasing_path_dp(matrix: Vec<Vec<i32>>) -> i32 {
    let rows = matrix.len();
    if rows == 0 { return 0; }
    let cols = matrix[0].len();
    if cols == 0 { return 0; }
    let mut h = vec![vec![0; cols + 2]; rows + 2];
    for i in 1..=rows {
        h[i][1..=cols].clone_from_slice(&matrix[i - 1])
    }
    let mut outdegree = vec![vec![0; cols + 2]; rows + 2];
    let dirs: Vec<(i32, i32)> = vec![(-1, 0), (0, -1), (1, 0), (0, 1)];
    for i in 1..=rows {
        for j in 1..=cols {
            for (d1, d2) in dirs.clone() {
                if h[i][j] < h[(i as i32 + d1) as usize][(j as i32 + d2) as usize] {
                    outdegree[i][j] += 1;
                }
            }
        }
    }
    let mut leaves = vec![];
    for i in 1..=rows {
        for j in 1..=cols {
            if outdegree[i][j] == 0 {
                leaves.push((i, j));
            }
        }
    }

    let mut height = 0;
    while !leaves.is_empty() {
        height += 1;
        let mut new_leaves = vec![];
        for (n1, n2) in leaves {
            for (d1, d2) in dirs.clone() {
                let x = (n1 as i32 + d1) as usize;
                let y = (n2 as i32 + d2) as usize;
                if h[n1][n2] > h[x][y] {
                    outdegree[x][y] -= 1;
                    if outdegree[x][y] == 0 {
                        new_leaves.push((x, y));
                    }
                }
            }
        }
        leaves = new_leaves;
    }
    height
}

fn main() {
    fn test(func: fn(matrix: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]]), 4);
        assert_eq!(func(vec![vec![3, 4, 5], vec![3, 2, 6], vec![2, 2, 1]]), 4);
        assert_eq!(func(vec![vec![1]]), 1);
    }
    test(longest_increasing_path);
    test(longest_increasing_path_dp);
}
