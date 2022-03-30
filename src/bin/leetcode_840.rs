//! 矩阵中的幻方

pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    if m < 3 || n < 3 { return 0; }
    let mut result = 0;
    const SORTED: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    fn check(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32, i: i32) -> bool {
        let mut sort = [a, b, c, d, e, f, g, h, i];
        sort.sort_unstable();
        sort == SORTED && (a + b + c == 15 && d + e + f == 15 && g + h + i == 15 && a + d + g == 15 &&
            b + e + h == 15 && c + f + i == 15 && a + e + i == 15 && c + e + g == 15)
    }
    for i in 0..=m - 3 {
        for j in 0..=n - 3 {
            if grid[i + 1][j + 1] != 5 { continue; }
            if check(grid[i][j], grid[i][j + 1], grid[i][j + 2],
                     grid[i + 1][j], grid[i + 1][j + 1], grid[i + 1][j + 2],
                     grid[i + 2][j], grid[i + 2][j + 1], grid[i + 2][j + 2]) {
                result += 1;
            }
        }
    }
    result
}

fn main() {
    assert_eq!(num_magic_squares_inside(vec![vec![4, 3, 8, 4], vec![9, 5, 1, 9], vec![2, 7, 6, 2]]), 1);
    assert_eq!(num_magic_squares_inside(vec![vec![8]]), 0);
}
