//! 矩形区域不超过 K 的最大数值和


use std::collections::BTreeSet;

pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut presum = vec![vec![0; n + 1]; m + 1];
    for i in 1..=m {
        for j in 1..=n {
            presum[i][j] = presum[i - 1][j] + presum[i][j - 1] + matrix[i - 1][j - 1] - presum[i - 1][j - 1];
        }
    }
    let mut result = i32::MIN;
    for x2 in 1..=m {
        for y2 in 1..=n {
            for x1 in 0..x2 {
                for y1 in 0..y2 {
                    let sum = presum[x2][y2] + presum[x1][y1] - presum[x2][y1] - presum[x1][y2];
                    if sum <= k {
                        result = result.max(sum);
                    }
                }
            }
        }
    }
    result
}

pub fn max_sum_submatrix_log(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut result = i32::MIN;
    for i in 0..m {
        let mut sum = vec![0; n];
        for j in i..m {
            for c in 0..n {
                sum[c] += matrix[j][c];
            }
            let mut set = BTreeSet::new();
            set.insert(0);
            let mut s = 0;
            for &v in &sum {
                s += v;
                if let Some(before) = set.range(s - k..).next() {
                    result = result.max(s - *before);
                }
                set.insert(s);
            }
        }
    }
    result
}

pub fn max_sum_submatrix_best(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    // 53. 最大子数组和
    fn dp_max(arr: &Vec<i32>, k: i32) -> i32 {
        // 要尽量大，就尽量不要负数
        let mut roll_sum = arr[0];
        let mut roll_max = roll_sum;
        let len = arr.len();
        for i in 1..len {
            if roll_sum > 0 {
                // 之前的和 > 0，那就累计进来
                roll_sum += arr[i];
            } else {
                // 之前的和 <= 0，那就重新开始
                roll_sum = arr[i];
            }
            if roll_sum > roll_max {
                roll_max = roll_sum;
            }
        }
        if roll_max <= k { return roll_max; }
        let mut result = i32::MIN;
        for i in 0..len {
            let mut sum = 0;
            for j in i..len {
                sum += arr[j];
                if sum > result && sum <= k {
                    result = sum;
                }
                if result == k { return k; }
            }
        }
        result
    }

    let m = matrix.len();
    let n = matrix[0].len();
    let mut result = i32::MIN;
    for i in 0..n {
        let mut row_sum = vec![0; m];
        for j in i..n {
            for k in 0..m {
                row_sum[k] += matrix[k][j];
            }
            result = result.max(dp_max(&row_sum, k));
            if result == k { return k; }
        }
    }
    result
}

fn main() {
    fn test(func: fn(matrix: Vec<Vec<i32>>, k: i32) -> i32) {
        assert_eq!(func(vec![vec![1, 0, 1], vec![0, -2, 3]], 2), 2);
        assert_eq!(func(vec![vec![2, 2, -1]], 3), 3);
    }
    test(max_sum_submatrix);
    test(max_sum_submatrix_log);
    test(max_sum_submatrix_best);
}
