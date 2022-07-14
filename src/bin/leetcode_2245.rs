//! 转角路径的乘积中最多能有几个尾随零

/// 前缀和
fn max_trailing_zeros(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut f2 = vec![vec![0; n + 1]; m + 1];
    let mut g2 = vec![vec![0; n + 1]; m + 1];
    let mut f5 = vec![vec![0; n + 1]; m + 1];
    let mut g5 = vec![vec![0; n + 1]; m + 1];
    for i in 1..=m {
        for j in 1..=n {
            let mut x = grid[i - 1][j - 1];
            let mut two = 0;
            let mut five = 0;
            while x % 2 == 0 {
                two += 1;
                x /= 2
            }
            while x % 5 == 0 {
                five += 1;
                x /= 5
            }
            f2[i][j] = f2[i][j - 1] + two;
            g2[i][j] = g2[i - 1][j] + two;
            f5[i][j] = f5[i][j - 1] + five;
            g5[i][j] = g5[i - 1][j] + five;
        }
    }
    let mut result = 0;
    for i in 1..=m {
        for j in 1..=n {
            // 从左边出发，到上边结束
            result = result.max((f2[i][j] + g2[i - 1][j]).min(f5[i][j] + g5[i - 1][j]));
            // 从左边出发，到下边结束
            result = result.max((f2[i][j] + g2[m][j] - g2[i][j]).min(f5[i][j] + g5[m][j] - g5[i][j]));
            // 从右边出发，到上边结束
            result = result.max((f2[i][n] - f2[i][j] + g2[i][j]).min(f5[i][n] - f5[i][j] + g5[i][j]));
            // 从右边出发，到下边结束
            result = result.max((f2[i][n] - f2[i][j] + g2[m][j] - g2[i - 1][j]).min(f5[i][n] - f5[i][j] + g5[m][j] - g5[i - 1][j]));
        }
    }
    return result;
}

fn main() {
    assert_eq!(max_trailing_zeros(vec![vec![284, 853, 142, 786, 199, 286], vec![910, 227, 820, 584, 593, 384], vec![519, 801, 66, 833, 587, 404], vec![360, 819, 518, 360, 16, 975], vec![145, 265, 177, 826, 219, 859], vec![410, 111, 353, 259, 902, 406]]), 6);
    assert_eq!(max_trailing_zeros(vec![vec![23, 17, 15, 3, 20], vec![8, 1, 20, 27, 11], vec![9, 4, 6, 2, 21], vec![40, 9, 1, 10, 6], vec![22, 7, 4, 5, 3]]), 3);
    assert_eq!(max_trailing_zeros(vec![vec![4, 3, 2], vec![7, 6, 1], vec![8, 8, 8]]), 0);
}
