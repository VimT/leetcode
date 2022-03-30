//! 用邮票贴满网格图

pub fn possible_to_stamp(grid: Vec<Vec<i32>>, stamp_height: i32, stamp_width: i32) -> bool {
    let m = grid.len();
    let n = grid[0].len();
    let h = stamp_height as usize;
    let w = stamp_width as usize;
    let mut prev = vec![vec![0; n + 1]; m + 1];
    for i in 1..=m {
        for j in 1..=n {
            prev[i][j] = prev[i - 1][j] + prev[i][j - 1] - prev[i - 1][j - 1] + grid[i - 1][j - 1];
        }
    }
    if prev[m][n] == (m * n) as i32 { return true; }
    if m < h || n < w { return false; }
    let mut diff = vec![vec![0; n + 1]; m + 1];

    for i in 0..=m - h {
        for j in 0..=n - w {
            if grid[i][j] == 0 {
                if prev[i + h][j + w] + prev[i][j] - prev[i + h][j] - prev[i][j + w] == 0 {
                    diff[i][j] += 1;
                    diff[i + h][j + w] += 1;
                    diff[i + h][j] -= 1;
                    diff[i][j + w] -= 1;
                }
            }
        }
    }
    let mut add = vec![vec![0; n + 1]; m + 1];
    for i in 1..=m {
        for j in 1..=n {
            // 对二维差分数组求前缀和（还原）
            add[i][j] = add[i - 1][j] + add[i][j - 1] - add[i - 1][j - 1] + diff[i - 1][j - 1];
            if grid[i - 1][j - 1] == 0 && add[i][j] == 0 {
                return false;
            }
        }
    }
    true
}

fn main() {
    assert_eq!(possible_to_stamp(vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]], 9, 4), true);
    assert_eq!(possible_to_stamp(vec![vec![1]], 1, 1), true);
    assert_eq!(possible_to_stamp(vec![vec![0]], 1, 1), true);
    assert_eq!(possible_to_stamp(vec![vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 1, 0, 0], vec![0, 0, 0, 0, 1], vec![0, 0, 0, 1, 1]], 2, 2), false);
    assert_eq!(possible_to_stamp(vec![vec![1, 0, 0, 0], vec![1, 0, 0, 0], vec![1, 0, 0, 0], vec![1, 0, 0, 0], vec![1, 0, 0, 0]], 4, 3), true);
    assert_eq!(possible_to_stamp(vec![vec![1, 0, 0, 0], vec![0, 1, 0, 0], vec![0, 0, 1, 0], vec![0, 0, 0, 1]], 2, 2), false);
}
