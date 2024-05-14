//! 人员站位的方案数 II

use std::collections::{HashMap, HashSet};

pub fn number_of_pairs(points: Vec<Vec<i32>>) -> i32 {
    // 离散化
    let mut m: HashSet<i32> = HashSet::new();
    for p in &points {
        m.insert(p[0]);
        m.insert(p[1]);
    }
    let mut m: Vec<i32> = m.into_iter().collect();
    m.sort_unstable();
    let m: HashMap<i32, usize> = m.into_iter().zip(0..).collect();
    let points = points.iter().map(|p| vec![m[&p[0]], m[&p[1]]]).collect::<Vec<_>>();

    // 二维前缀和 （以左下为原点）
    let mut s = vec![vec![0; m.len()]; m.len()];
    for p in &points {
        s[p[0]][p[1]] = 1;
    }
    let mut presum = vec![vec![0; m.len() + 1]; m.len() + 1];
    for i in 1..=m.len() {
        for j in 1..=m.len() {
            presum[i][j] = presum[i - 1][j] + presum[i][j - 1] - presum[i - 1][j - 1] + s[i - 1][j - 1];
        }
    }

    let mut result = 0;
    let len = points.len();
    for i in 0..len {
        for j in i + 1..len {
            let (mut a, mut b) = (points[i][0], points[i][1]);
            let (mut c, mut d) = (points[j][0], points[j][1]);
            if (c <= a) && b <= d {
                let tmp = (a, b);
                a = c;
                b = d;
                c = tmp.0;
                d = tmp.1;
            }
            if c >= a && b >= d {
                let (a, b, c, d) = (a, d, c, b);
                let cnt = presum[c + 1][d + 1] - presum[a][d + 1] - presum[c + 1][b] + presum[a][b];
                if cnt == 2 {
                    result += 1;
                }
            }
        }
    }

    result
}

fn main() {
    fn test(func: fn(points: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![0, 1], vec![6, 1]]), 1);
        assert_eq!(func(vec![vec![1, 1], vec![2, 2], vec![3, 3]]), 0);
        assert_eq!(func(vec![vec![6, 2], vec![4, 4], vec![2, 6]]), 2);
        assert_eq!(func(vec![vec![3, 1], vec![1, 3], vec![1, 1]]), 2);
    }
    test(number_of_pairs);
}
