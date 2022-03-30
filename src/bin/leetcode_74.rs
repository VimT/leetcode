//! 搜索二维矩阵

pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let m = matrix.len();
    if m == 0 { return false; }
    let n = matrix[0].len();
    if n == 0 { return false; }

    let mut left = 0;
    let mut right = m;
    while left < right {
        let mid = left + (right - left) / 2;
        if matrix[mid][0] >= target {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    if matrix[left][0] == target { return true; }
    if left == 0 { return false; }
    return matrix[left - 1].binary_search(&target).is_ok();
}

pub fn search_matrix_by_key(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let m = matrix.len();
    if m == 0 { return false; }
    let n = matrix[0].len();
    if n == 0 { return false; }
    return match matrix.binary_search_by_key(&target, |x| x[0]) {
        Ok(_) => { true }
        Err(key) => {
            if key == 0 { return false; }
            matrix[key - 1].binary_search(&target).is_ok()
        }
    };
}

fn main() {
    fn test(func: fn(matrix: Vec<Vec<i32>>, target: i32) -> bool) {
        assert_eq!(func(vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]], 3), true);
        assert_eq!(func(vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]], 13), false);
    }
    test(search_matrix);
    test(search_matrix_by_key);
}
