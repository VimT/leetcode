//! 最强祝福力场


use std::collections::{HashMap, HashSet};

/// 二维差分
pub fn field_of_greatest_blessing(force_field: Vec<Vec<i32>>) -> i32 {
    let mut xl = HashSet::new();
    let mut yl = HashSet::new();
    let field: Vec<[(i64, i64); 2]> = force_field.into_iter().map(|field| {
        let (x, y, len) = (field[0] as i64 * 2, field[1] as i64 * 2, field[2] as i64);
        [(x - len, y - len), (x + len, y + len)]
    }).collect();
    for f in &field {
        for &(x, y) in f {
            xl.insert(x);
            yl.insert(y);
        }
    }
    let mut xlist: Vec<i64> = xl.into_iter().collect();
    let mut ylist: Vec<i64> = yl.into_iter().collect();
    xlist.sort_unstable();
    ylist.sort_unstable();
    let xm: HashMap<i64, usize> = xlist.into_iter().zip(0..).collect();
    let ym: HashMap<i64, usize> = ylist.into_iter().zip(0..).collect();
    let m = xm.len();
    let n = ym.len();
    let mut diff = vec![vec![0; n + 1]; m + 1];
    for f in &field {
        let (x1, y1) = f[0];
        let (x2, y2) = f[1];
        let (x1, y1) = (xm[&x1], ym[&y1]);
        let (x2, y2) = (xm[&x2], ym[&y2]);
        diff[x1][y1] += 1;
        diff[x2 + 1][y1] -= 1;
        diff[x1][y2 + 1] -= 1;
        diff[x2 + 1][y2 + 1] += 1;
    }
    let mut result = 0;
    let mut presum = vec![vec![0; n + 1]; m + 1];
    for i in 0..m {
        for j in 0..n {
            presum[i + 1][j + 1] = diff[i][j] + presum[i][j + 1] + presum[i + 1][j] - presum[i][j];
            result = result.max(presum[i + 1][j + 1]);
        }
    }
    result
}

fn main() {
    fn test(func: fn(force_field: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![0, 0, 1], vec![1, 0, 1]]), 2);
        assert_eq!(func(vec![vec![4, 4, 6], vec![7, 5, 3], vec![1, 6, 2], vec![5, 6, 3]]), 3);
    }
    test(field_of_greatest_blessing);
}
