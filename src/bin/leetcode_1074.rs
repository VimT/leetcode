//! 元素和为目标值的子矩阵数量


pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut p = vec![vec![0; n + 1]; m + 1];
    for i in 1..=m {
        for j in 1..=n {
            p[i][j] = p[i - 1][j] + p[i][j - 1] - p[i - 1][j - 1] + matrix[i - 1][j - 1];
        }
    }
    let mut ans = 0;
    for x1 in 0..m {
        for y1 in 0..n {
            for x2 in x1..m {
                for y2 in y1..n {
                    if p[x2 + 1][y2 + 1] + p[x1][y1] - p[x2 + 1][y1] - p[x1][y2 + 1] == target {
                        ans += 1;
                    }
                }
            }
        }
    }

    ans
}

pub fn num_submatrix_sum_target_hash(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
    /// leetcode_560
    fn cal(num: &Vec<i32>, target: i32) -> i32 {
        let mut m = std::collections::HashMap::new();
        let len = num.len();
        let mut pre = 0;
        let mut ans = 0;
        m.insert(0, 1);
        for i in 0..len {
            pre += num[i];
            ans += *m.get(&(pre - target)).unwrap_or(&0);
            *m.entry(pre).or_insert(0) += 1;
        }
        ans
    }

    let m = matrix.len();
    let n = matrix[0].len();
    let mut ans = 0;
    for r1 in 0..m {
        let mut row = vec![0; n];
        for r2 in r1..m {
            for j in 0..n {
                row[j] += matrix[r2][j];
            }
            ans += cal(&row, target);
        }
    }
    ans
}


fn main() {
    fn test(func: fn(matrix: Vec<Vec<i32>>, target: i32) -> i32) {
        assert_eq!(func(vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 1, 0]], 0), 4);
        assert_eq!(func(vec![vec![1, -1], vec![-1, 1]], 0), 5);
        assert_eq!(func(vec![vec![904]], 0), 0);
    }
    test(num_submatrix_sum_target);
    test(num_submatrix_sum_target_hash);
}
