//! 无重叠区间

/// 贪心,不断重复，找右侧最小的区间
pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
    let len = intervals.len();
    if len == 0 { return 0; }
    intervals.sort_unstable_by_key(|x| x[1]);
    let mut count = 1;
    let mut last_right = intervals[0][1];
    for i in 1..len {
        if intervals[i][0] >= last_right {
            last_right = intervals[i][1];
            count += 1;
        }
    }
    (len - count) as i32
}

/// 本质上是一个「最长上升子序列」问题, 300
pub fn erase_overlap_intervals_dp(mut intervals: Vec<Vec<i32>>) -> i32 {
    intervals.sort_unstable();
    let len = intervals.len();
    if len == 0 { return 0; }
    let mut dp = vec![1; len];
    dp[0] = 1;
    for i in 1..len {
        for j in 0..i {
            if intervals[j][1] <= intervals[i][0] {
                dp[i] = dp[i].max(dp[j] + 1);
            }
        }
    }
    (len - *dp.iter().max().unwrap()) as i32
}


fn main() {
    fn test(func: fn(intervals: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]]), 1);
        assert_eq!(func(vec![vec![1, 2], vec![1, 2], vec![1, 2]]), 2);
        assert_eq!(func(vec![vec![1, 2], vec![2, 3]]), 0);
    }
    test(erase_overlap_intervals);
    test(erase_overlap_intervals_dp);
}
