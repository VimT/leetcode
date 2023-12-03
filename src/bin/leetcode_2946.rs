//! 循环移位后的矩阵相似检查

pub fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
    let m = mat.len();
    let n = mat[0].len();
    let k = k as usize % n;
    if k == 0 { return true; }
    for i in 0..m {
        // 不用管左移还是右移
        for j in 0..n {
            if mat[i][j] != mat[i][(j + k) % n] {
                return false;
            }
        }
    }
    true
}

fn main() {
    fn test(func: fn(mat: Vec<Vec<i32>>, k: i32) -> bool) {
        assert_eq!(func(vec![vec![1, 2, 1, 2], vec![5, 5, 5, 5], vec![6, 3, 6, 3]], 2), true);
        assert_eq!(func(vec![vec![2, 2], vec![2, 2]], 3), true);
        assert_eq!(func(vec![vec![1, 2]], 1), false);
    }
    test(are_similar);
}
