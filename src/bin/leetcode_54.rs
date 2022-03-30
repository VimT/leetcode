//! 螺旋矩阵


/// 每次遍历一个环
pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    if matrix.len() == 0 { return vec![]; }
    let m = matrix.len();
    let n = matrix[0].len();
    let sum = m * n;

    let ring = m.min(n) / 2;
    let mut ans = vec![];
    for i in 0..=ring {
        for j in i..n - i {
            ans.push(matrix[i][j]);
            if ans.len() == sum { return ans; }
        }
        for j in i + 1..m - i {
            ans.push(matrix[j][n - i - 1]);
            if ans.len() == sum { return ans; }
        }
        for j in (i..n - i - 1).rev() {
            ans.push(matrix[m - i - 1][j]);
            if ans.len() == sum { return ans; }
        }
        for j in (i + 1..m - i - 1).rev() {
            ans.push(matrix[j][i]);
            if ans.len() == sum { return ans; }
        }
    }
    ans
}

/// 使用两个变量，易理解
pub fn spiral_order_optimise(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ans = vec![];
    if matrix.len() == 0 { return ans; }
    let m = matrix.len();
    let n = matrix[0].len();
    let mut r1 = 0;
    let mut r2 = m - 1;
    let mut c1 = 0;
    let mut c2 = n - 1;
    while r1 <= r2 && c1 <= c2 {
        for c in c1..=c2 { ans.push(matrix[r1][c]); }
        for r in r1 + 1..=r2 { ans.push(matrix[r][c2]); }
        if r1 < r2 && c1 < c2 {
            for c in (c1 + 1..=c2 - 1).rev() { ans.push(matrix[r2][c]); }
            for r in (r1 + 1..=r2).rev() { ans.push(matrix[r][c1]); }
        }
        r1 += 1;
        r2 -= 1;
        c1 += 1;
        c2 -= 1;
    }
    ans
}


fn main() {
    fn test(func: fn(matrix: Vec<Vec<i32>>) -> Vec<i32>) {
        assert_eq!(func(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]), vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);
        assert_eq!(func(vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]]), vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]);
    }
    test(spiral_order);
    test(spiral_order_optimise);
}
