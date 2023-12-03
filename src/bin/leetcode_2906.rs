//! 构造乘积矩阵

pub fn construct_product_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = grid.len();
    let n = grid[0].len();
    const MOD: i64 = 12345;
    let mut suffix = vec![1; n * m + 1];
    let mut num = 1;
    for i in (0..m).rev() {
        for j in (0..n).rev() {
            suffix[i * n + j] = num;
            num = (num * grid[i][j] as i64) % MOD;
        }
    }
    let mut result = vec![vec![1; n]; m];
    num = 1;
    for i in 0..m {
        for j in 0..n {
            result[i][j] = ((suffix[i * n + j] * num) % MOD) as i32;
            num = (num * grid[i][j] as i64) % MOD;
        }
    }
    result
}

fn main() {
    fn test(func: fn(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>>) {
        assert_eq!(func(vec![vec![1, 2], vec![3, 4]]), vec![vec![24, 12], vec![8, 6]]);
        assert_eq!(func(vec![vec![12345], vec![2], vec![1]]), vec![vec![2], vec![0], vec![0]]);
    }
    test(construct_product_matrix);
}
