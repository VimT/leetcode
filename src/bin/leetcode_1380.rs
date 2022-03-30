//! 矩阵中的幸运数

pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let n = matrix.len();
    let m = matrix[0].len();
    let mut ans = vec![];

    'a: for row in 0..n {
        let mut min = matrix[row][0];
        let mut min_col = 0;
        for col in 1..m {
            if matrix[row][col] < min {
                min = matrix[row][col];
                min_col = col;
            }
        }

        for i in 0..n {
            if matrix[i][min_col] > min {
                continue 'a;
            }
        }
        ans.push(min);
    }

    ans
}


fn main() {
    assert_eq!(lucky_numbers(vec![vec![3, 7, 8], vec![9, 11, 13], vec![15, 16, 17]]), vec![15]);
    assert_eq!(lucky_numbers(vec![vec![1, 10, 4, 2], vec![9, 3, 8, 7], vec![15, 16, 17, 12]]), vec![12]);
    assert_eq!(lucky_numbers(vec![vec![7, 8], vec![1, 2]]), vec![7]);
}
