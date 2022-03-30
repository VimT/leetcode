//! 01 矩阵


pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = mat.len();
    let n = mat[0].len();
    let mut result = vec![vec![i32::MAX / 2; n]; m];
    for i in 0..m { for j in 0..n { if mat[i][j] == 0 { result[i][j] = 0; } } }
    for i in 0..m {
        for j in 0..n {
            if i > 0 { result[i][j] = result[i][j].min(result[i - 1][j] + 1); }
            if j > 0 { result[i][j] = result[i][j].min(result[i][j - 1] + 1); }
        }
    }
    for i in (0..m).rev() {
        for j in (0..n).rev() {
            if i + 1 < m { result[i][j] = result[i][j].min(result[i + 1][j] + 1); }
            if j + 1 < n { result[i][j] = result[i][j].min(result[i][j + 1] + 1); }
        }
    }
    result
}

fn main() {
    assert_eq!(update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]), vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]);
    assert_eq!(update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]]), vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]]);
}
