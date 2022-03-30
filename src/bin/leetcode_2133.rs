//! 检查是否每一行每一列都包含全部整数

pub fn check_valid(matrix: Vec<Vec<i32>>) -> bool {
    let n = matrix.len();
    for i in 0..n {
        let mut vis = vec![false; n];
        for j in 0..n {
            vis[matrix[i][j] as usize - 1] = true;
        }
        for i in 0..n {
            if !vis[i] { return false; }
        }
    }
    for j in 0..n {
        let mut vis = vec![false; n];
        for i in 0..n {
            vis[matrix[i][j] as usize - 1] = true;
        }
        for i in 0..n {
            if !vis[i] { return false; }
        }
    }
    true
}

fn main() {
    assert_eq!(check_valid(vec![vec![2, 2, 2], vec![2, 2, 2], vec![2, 2, 2]]), false);
    assert_eq!(check_valid(vec![vec![1, 2, 3], vec![3, 1, 2], vec![2, 3, 1]]), true);
    assert_eq!(check_valid(vec![vec![1, 1, 1], vec![1, 2, 3], vec![1, 2, 3]]), false);
}
