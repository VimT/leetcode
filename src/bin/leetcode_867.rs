//! 转置矩阵

pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut result = vec![vec![0; m]; n];
    for i in 0..m {
        for j in 0..n {
            result[j][i] = matrix[i][j];
        }
    }
    result
}

fn main() {
    assert_eq!(transpose(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]), vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]);
    assert_eq!(transpose(vec![vec![1, 2, 3], vec![4, 5, 6]]), vec![vec![1, 4], vec![2, 5], vec![3, 6]]);
}
