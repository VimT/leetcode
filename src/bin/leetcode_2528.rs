//! 最大化城市的最小供电站数目

/// 二分+贪心
/// 可以用前缀和+查分数组再优化
pub fn max_power(stations: Vec<i32>, r: i32, k: i32) -> i64 {
    let r = r as usize;
    let stations: Vec<i64> = stations.into_iter().map(|x| x as i64).collect();
    let len = stations.len();
    let mut cur = 0;
    for i in 0..(2 * r + 1).min(len) {
        cur += stations[i];
    }
    let mut max = cur;
    for i in 2 * r + 1..len {
        cur += stations[i];
        cur -= stations[i - 2 * r - 1];
        max = max.max(cur);
    }
    let mut left = 0;
    let mut right = max + k as i64 + 1;
    while left < right {
        let mid = (left + right) / 2;
        let mut cur = 0;
        let mut stations = stations.clone();
        for i in 0..=r {
            cur += stations[i];
        }
        let mut k = k as i64;
        for i in 0..len {
            // 当前这个位置的不够用，在最右边加
            if cur < mid {
                k -= mid - cur;
                if k < 0 { break; }
                stations[(i + r).min(len - 1)] += mid - cur;
                cur = mid;
            }
            // 下一个
            if i + 1 + r < len {
                cur += stations[i + 1 + r];
            }
            if i >= r {
                cur -= stations[i - r];
            }
        }
        if k >= 0 {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left - 1
}

fn main() {
    fn test(func: fn(stations: Vec<i32>, r: i32, k: i32) -> i64) {
        assert_eq!(func(vec![13, 12, 8, 14, 7], 2, 23), 52);
        assert_eq!(func(vec![4, 2], 1, 1), 7);
        assert_eq!(func(vec![1, 2, 4, 5, 0], 1, 2), 5);
        assert_eq!(func(vec![4, 4, 4, 4], 0, 3), 4);
    }
    test(max_power);
}
