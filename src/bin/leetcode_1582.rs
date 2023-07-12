//! 二进制矩阵中的特殊位置

pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
    let m = mat.len();
    let n = mat[0].len();

    let mut row_sum = vec![0; m];
    let mut col_sum = vec![0; n];
    for i in 0..m {
        row_sum[i] = mat[i].iter().sum();
    }
    for j in 0..n {
        col_sum[j] = (0..m).map(|i| mat[i][j]).sum();
    }
    let mut result = 0;
    for i in 0..m {
        for j in 0..n {
            if mat[i][j] == 1 && col_sum[j] == 1 && row_sum[i] == 1 {
                result += 1;
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(mat: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![1, 0, 0], vec![0, 0, 1], vec![1, 0, 0]]), 1);
        assert_eq!(func(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]), 3);
        assert_eq!(func(vec![vec![0, 0, 0, 1], vec![1, 0, 0, 0], vec![0, 1, 1, 0], vec![0, 0, 0, 0]]), 2);
        assert_eq!(func(vec![vec![0, 0, 0, 0, 0], vec![1, 0, 0, 0, 0], vec![0, 1, 0, 0, 0], vec![0, 0, 1, 0, 0], vec![0, 0, 0, 1, 1]]), 3);
    }
    test(num_special);
}
