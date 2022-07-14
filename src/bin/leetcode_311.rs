//! 稀疏矩阵的乘法

/// 正常矩阵乘法
pub fn multiply(mat1: Vec<Vec<i32>>, mat2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = mat1.len();
    let k = mat1[0].len();
    let n = mat2[0].len();
    let mut result = vec![vec![0; n]; m];
    for i in 0..m {
        for j in 0..n {
            for l in 0..k {
                result[i][j] += mat1[i][l] * mat2[l][j];
            }
        }
    }
    result
}

/// 稀疏三元组矩阵乘法
pub fn multiply2(mat1: Vec<Vec<i32>>, mat2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = mat1.len();
    let n = mat2[0].len();
    fn get_sparse_representation(matrix: Vec<Vec<i32>>) -> Vec<(i32, usize, usize)> {
        matrix.iter().enumerate().flat_map(|(i, row)| {
            row.iter().enumerate().map(move |(j, val)| (*val, i, j))
        }).collect()
    }
    let pos1 = get_sparse_representation(mat1);
    let pos2 = get_sparse_representation(mat2);
    let mut result = vec![vec![0; n]; m];
    for (val1, x1, y1) in pos1 {
        for &(val2, x2, y2) in &pos2 {
            if y1 == x2 {
                result[x1][y2] += val1 * val2;
            }
        }
    }
    return result;
}

fn main() {
    fn test(func: fn(mat1: Vec<Vec<i32>>, mat2: Vec<Vec<i32>>) -> Vec<Vec<i32>>) {
        assert_eq!(func(vec![vec![1, 0, 0], vec![-1, 0, 3]], vec![vec![7, 0, 0], vec![0, 0, 0], vec![0, 0, 1]]), vec![vec![7, 0, 0], vec![-7, 0, 3]]);
        assert_eq!(func(vec![vec![0]], vec![vec![0]]), vec![vec![0]]);
    }
    test(multiply);
    test(multiply2);
}
