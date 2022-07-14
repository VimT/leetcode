//! 最小化去加油站的最大距离

/// 二分查找
pub fn minmax_gas_dist(stations: Vec<i32>, k: i32) -> f64 {
    let mut left = 0f64;
    let mut right = 1e8 as f64;
    while right - left > 1e-6 {
        let mid = (left + right) / 2.;
        let mut used = 0;
        for i in 0..stations.len() - 1 {
            used += ((stations[i + 1] - stations[i]) as f64 / mid) as i32;
        }
        if used <= k {
            right = mid;
        } else {
            left = mid;
        }
    }
    left
}

fn main() {
    fn test(func: fn(stations: Vec<i32>, k: i32) -> f64) {
        assert_eq!((func(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 9) - 0.50000).abs() < 1e-6, true);
        assert_eq!((func(vec![23, 24, 36, 39, 46, 56, 57, 65, 84, 98], 1) - 14.00000).abs() < 1e-6, true);
    }
    test(minmax_gas_dist);
}
