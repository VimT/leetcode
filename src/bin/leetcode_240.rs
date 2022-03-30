//! 搜索二维矩阵 II

pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut x = 0;
    let mut y = n - 1;
    while x < m {
        if matrix[x][y] > target {
            if y == 0 { return false; }
            y -= 1;
        } else if matrix[x][y] < target {
            x += 1;
        } else {
            return true;
        }
    }
    false
}

fn main() {
    assert_eq!(search_matrix(vec![vec![1, 4, 7, 11, 15], vec![2, 5, 8, 12, 19], vec![3, 6, 9, 16, 22], vec![10, 13, 14, 17, 24], vec![18, 21, 23, 26, 30]], 5), true);
    assert_eq!(search_matrix(vec![vec![1, 4, 7, 11, 15], vec![2, 5, 8, 12, 19], vec![3, 6, 9, 16, 22], vec![10, 13, 14, 17, 24], vec![18, 21, 23, 26, 30]], 20), false);
}
