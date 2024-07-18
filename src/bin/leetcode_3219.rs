//! 切蛋糕的最小总开销 II

/// 贪心
pub fn minimum_cost(m: i32, n: i32, mut h: Vec<i32>, mut v: Vec<i32>) -> i64 {
    h.sort_unstable_by(|a, b| b.cmp(a));
    v.sort_unstable_by(|a, b| b.cmp(a));
    let mut result = 0;
    let (mut i, mut j) = (0, 0);
    let (m, n) = (m as usize - 1, n as usize - 1);
    while i < m || j < n {
        if j == n || i < m && h[i] > v[j] {
            result += h[i] as i64 * (j + 1) as i64; // 横切
            i += 1;
        } else {
            result += v[j] as i64 * (i + 1) as i64; // 竖切
            j += 1;
        }
    }
    result
}

fn main() {
    fn test(func: fn(m: i32, n: i32, horizontal_cut: Vec<i32>, vertical_cut: Vec<i32>) -> i64) {
        assert_eq!(func(3, 2, vec![1, 3], vec![5]), 13);
        assert_eq!(func(2, 2, vec![7], vec![4]), 15);
    }
    test(minimum_cost);
}
