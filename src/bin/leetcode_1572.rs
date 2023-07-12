//! 矩阵对角线元素的和

pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
    let len = mat.len();
    let mut result = 0;
    for i in 0..len {
        let (c, d) = (i, len - 1 - i);
        result += mat[i][i];
        if (c, d) != (i, i) {
            result += mat[c][d];
        }
    }
    result
}

fn main() {
    fn test(func: fn(mat: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]), 25);
        assert_eq!(func(vec![vec![1, 1, 1, 1], vec![1, 1, 1, 1], vec![1, 1, 1, 1], vec![1, 1, 1, 1]]), 8);
        assert_eq!(func(vec![vec![5]]), 5);
    }
    test(diagonal_sum);
}
