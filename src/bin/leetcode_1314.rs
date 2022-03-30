//! 矩阵区域和

pub fn matrix_block_sum(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let m = mat.len();
    let n = mat[0].len();

    let mut presum = vec![vec![0; n + 1]; m + 1];
    for i in 1..=m {
        for j in 1..=n {
            presum[i][j] = presum[i][j - 1] + presum[i - 1][j] + mat[i - 1][j - 1] - presum[i - 1][j - 1];
        }
    }
    let k = k as usize;
    let mut result = vec![vec![0; n]; m];
    for i in 0..m {
        for j in 0..n {
            let a = (i + k + 1).min(m);
            let b = (j + k + 1).min(n);
            let c = if i >= k { i - k } else { 0 };
            let d = if j >= k { j - k } else { 0 };
            result[i][j] = presum[a][b] + presum[c][d] - presum[a][d] - presum[c][b];
        }
    }
    result
}


fn main() {
    assert_eq!(matrix_block_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 1), vec![vec![12, 21, 16], vec![27, 45, 33], vec![24, 39, 28]]);
    assert_eq!(matrix_block_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 2), vec![vec![45, 45, 45], vec![45, 45, 45], vec![45, 45, 45]]);
}
