//! 给定行和列的和求可行矩阵

pub fn restore_matrix(mut row_sum: Vec<i32>, mut col_sum: Vec<i32>) -> Vec<Vec<i32>> {
    let m = row_sum.len();
    let n = col_sum.len();
    let mut result = vec![vec![0; n]; m];
    for i in 0..m {
        for j in 0..n {
            result[i][j] = row_sum[i].min(col_sum[j]);
            row_sum[i] -= result[i][j];
            col_sum[j] -= result[i][j];
        }
    }
    result
}

fn main() {
    fn test(func: fn(row_sum: Vec<i32>, col_sum: Vec<i32>) -> Vec<Vec<i32>>) {
        fn verify(func: fn(row_sum: Vec<i32>, col_sum: Vec<i32>) -> Vec<Vec<i32>>, row_sum: Vec<i32>, col_sum: Vec<i32>) {
            let result = func(row_sum.clone(), col_sum.clone());
            for i in 0..row_sum.len() {
                assert_eq!(result[i].iter().sum::<i32>(), row_sum[i]);
            }
            for i in 0..col_sum.len() {
                assert_eq!(result.iter().map(|x| x[i]).sum::<i32>(), col_sum[i]);
            }
        }
        verify(func, vec![3, 8], vec![4, 7]);
        verify(func, vec![5, 7, 10], vec![8, 6, 8]);
    }
    test(restore_matrix);
}
