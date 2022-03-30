//! 二维区域和检索 - 矩阵不可变

struct NumMatrix {
    s: Vec<Vec<i32>>,
}

impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let m = matrix.len();
        if m == 0 { return NumMatrix { s: vec![vec![0; 1]; 1] }; }
        let n = matrix[0].len();
        let mut s = vec![vec![0; n + 1]; m + 1];
        for i in 1..=m {
            for j in 1..=n {
                s[i][j] = s[i - 1][j] + s[i][j - 1] - s[i - 1][j - 1] + matrix[i - 1][j - 1];
            }
        }
        NumMatrix { s }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let (i, j, m, n) = (row1 as usize, col1 as usize, row2 as usize + 1, col2 as usize + 1);
        self.s[m][n] + self.s[i][j] - self.s[i][n] - self.s[m][j]
    }
}

fn main() {
    let nm = NumMatrix::new(vec![
        vec![3, 0, 1, 4, 2],
        vec![5, 6, 3, 2, 1],
        vec![1, 2, 0, 1, 5],
        vec![4, 1, 0, 1, 7],
        vec![1, 0, 3, 0, 5],
    ]);
    assert_eq!(nm.sum_region(2, 1, 4, 3), 8);
    assert_eq!(nm.sum_region(1, 1, 2, 2), 11);
    assert_eq!(nm.sum_region(1, 2, 2, 4), 12);

    NumMatrix::new(vec![]);
}
