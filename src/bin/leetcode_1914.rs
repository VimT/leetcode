//! 循环轮转矩阵

pub fn rotate_grid(mut grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let m = grid.len();
    let n = grid[0].len();
    let round = m.min(n) / 2;
    for i in 0..round {
        let zhou = (m + n - 2 - 4 * i) * 2;
        let k = k as usize % zhou;
        let mut v = Vec::with_capacity(zhou);
        for x in i..m - i {
            v.push(grid[x][i]);
        }
        for y in i + 1..n - i {
            v.push(grid[m - i - 1][y]);
        }
        for x in (i..m - i - 1).rev() {
            v.push(grid[x][n - i - 1]);
        }
        for y in (i + 1..n - i - 1).rev() {
            v.push(grid[i][y]);
        }
        let mut cur = zhou - k;
        for x in i..m - i {
            grid[x][i] = v[cur % zhou];
            cur += 1;
        }
        for y in i + 1..n - i {
            grid[m - i - 1][y] = v[cur % zhou];
            cur += 1;
        }
        for x in (i..m - i - 1).rev() {
            grid[x][n - i - 1] = v[cur % zhou];
            cur += 1;
        }
        for y in (i + 1..n - i - 1).rev() {
            grid[i][y] = v[cur % zhou];
            cur += 1;
        }
    }
    grid
}

fn main() {
    assert_eq!(rotate_grid(vec![vec![40, 10], vec![30, 20]], 1), vec![vec![10, 20], vec![40, 30]]);
    assert_eq!(rotate_grid(vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12], vec![13, 14, 15, 16]], 2), vec![vec![3, 4, 8, 12], vec![2, 11, 10, 16], vec![1, 7, 6, 15], vec![5, 9, 13, 14]]);
}
