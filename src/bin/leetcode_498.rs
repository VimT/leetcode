//! 对角线遍历

pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
    let m = mat.len();
    let n = mat[0].len();
    let mut result = Vec::with_capacity(m * n);
    for sum in 0..m + n - 1 {
        let mut inter = Vec::with_capacity(sum);
        let mut r = if sum < n { 0 } else { sum - n + 1 };
        let mut c = if sum < n { sum } else { n - 1 };
        while r < m {
            inter.push(mat[r][c]);
            r += 1;
            if c == 0 { break; }
            c -= 1;
        }
        if sum & 1 == 0 {
            inter.reverse();
        }
        result.extend(inter);
    }
    result
}

fn main() {
    assert_eq!(find_diagonal_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]), vec![1, 2, 4, 7, 5, 3, 6, 8, 9]);
    assert_eq!(find_diagonal_order(vec![vec![1, 2], vec![3, 4]]), vec![1, 2, 3, 4]);
}
