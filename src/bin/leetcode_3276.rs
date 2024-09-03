//! 选择矩阵中单元格的最大得分

/// 贪心，总是选当前行最大的数，如果当前行最大的数已经选过了，就选次大的数
pub fn max_score(mut grid: Vec<Vec<i32>>) -> i32 {
    fn dfs(grid: &Vec<Vec<i32>>, i: usize, used: u128) -> i32 {
        if i == grid.len() { return 0; }
        let mut result = 0;
        for &num in &grid[i] {
            if used >> num & 1 == 1 { continue; }
            let sub = dfs(grid, i + 1, used | 1 << num);
            let this = num + sub;
            if this < result {
                break;
            }
            result = this;
        }
        if result == 0 {
            return dfs(grid, i + 1, used);
        }
        result
    }
    for line in &mut grid {
        line.sort_unstable_by(|a, b| b.cmp(a))
    }
    grid.sort_unstable();
    dfs(&grid, 0, 0)
}

/// 状态压缩 dp
/// dp[i][j] 表示当前在 [1, i] 里面选数字，选的数字的行不能在 j 里面
pub fn max_score2(grid: Vec<Vec<i32>>) -> i32 {
    let mx = *grid.iter().map(|line| line.iter().max().unwrap()).max().unwrap();
    let mut pos = vec![0; mx as usize + 1];
    let len = grid.len();
    for (i, line) in grid.into_iter().enumerate() {
        for num in line {
            pos[num as usize] |= 1 << i;
        }
    }
    fn dfs(pos: &Vec<i32>, len: usize, i: usize, j: i32, mem: &mut Vec<Vec<i32>>) -> i32 {
        if i == 0 { return 0; }
        if mem[i][j as usize] != -1 { return mem[i][j as usize]; }

        let mut result = dfs(pos, len, i - 1, j, mem);
        let g = pos[i] & !j;
        let mut s = g;
        while s > 0 {
            let k = s.trailing_zeros() as usize;
            result = result.max(i as i32 + dfs(pos, len, i - 1, j | (1 << k), mem));
            s = g & (s - 1);
        }
        mem[i][j as usize] = result;
        result
    }
    dfs(&pos, len, mx as usize, 0, &mut vec![vec![-1; 1 << len]; mx as usize + 1])
}

/// dfs 改递推
pub fn max_score3(grid: Vec<Vec<i32>>) -> i32 {
    let mx = *grid.iter().map(|line| line.iter().max().unwrap()).max().unwrap();
    let mut pos = vec![0; mx as usize + 1];
    let len = grid.len();
    for (i, line) in grid.into_iter().enumerate() {
        for num in line {
            pos[num as usize] |= 1 << i;
        }
    }
    let mut dp = vec![vec![0; 1 << len]; mx as usize + 1];
    for i in 1..=mx as usize {
        for j in 0..1 << len {
            dp[i][j] = dp[i - 1][j];
            let g = pos[i] & !j;
            let mut s = g;
            while s > 0 {
                let k = s.trailing_zeros() as usize;
                dp[i][j] = dp[i][j].max(i as i32 + dp[i - 1][j | (1 << k)]);
                s = g & (s - 1);
            }
        }
    }
    dp[mx as usize][0]
}

/// 空间优化
pub fn max_score4(grid: Vec<Vec<i32>>) -> i32 {
    let mx = *grid.iter().map(|line| line.iter().max().unwrap()).max().unwrap();
    let mut pos = vec![0usize; mx as usize + 1];
    let len = grid.len();
    for (i, line) in grid.into_iter().enumerate() {
        for num in line {
            pos[num as usize] |= 1 << i;
        }
    }
    let mut dp = vec![0; 1 << len];
    for i in 1..=mx as usize {
        for j in 0..1 << len {
            let g = pos[i] & !j;
            let mut s = g;
            while s > 0 {
                let k = s.trailing_zeros() as usize;
                dp[j] = dp[j].max(i as i32 + dp[j | (1 << k)]);
                s = g & (s - 1);
            }
        }
    }
    dp[0]
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![5], vec![7], vec![19], vec![5]]), 31);
        assert_eq!(func(vec![vec![16, 18], vec![20, 20], vec![18, 18], vec![1, 15]]), 69);
        assert_eq!(func(vec![vec![1, 2, 3], vec![4, 3, 2], vec![1, 1, 1]]), 8);
        assert_eq!(func(vec![vec![8, 7, 6], vec![8, 3, 2]]), 15);
    }
    test(max_score);
    test(max_score2);
    test(max_score3);
    test(max_score4);
}
