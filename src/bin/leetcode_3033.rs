//! 修改矩阵

pub fn modified_matrix(mut matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut col_max = vec![0; n];
    for i in 0..m {
        for j in 0..n {
            col_max[j] = col_max[j].max(matrix[i][j]);
        }
    }
    for i in 0..m {
        for j in 0..n {
            if matrix[i][j] == -1 {
                matrix[i][j] = col_max[j];
            }
        }
    }
    matrix
}

fn main() {
    fn test(func: fn(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>>) {
        assert_eq!(func(vec![vec![1, 2, -1], vec![4, -1, 6], vec![7, 8, 9]]), vec![vec![1, 2, 9], vec![4, 8, 6], vec![7, 8, 9]]);
        assert_eq!(func(vec![vec![3, -1], vec![5, 2]]), vec![vec![3, 2], vec![5, 2]]);
    }
    test(modified_matrix);
}
