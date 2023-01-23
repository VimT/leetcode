//! 灌溉花园的最少水龙头数目

/// dp[i] 表示覆盖前i需要的最小个数
pub fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
    let n = n as usize;
    let mut dp = vec![i32::MAX / 2; n + 1];
    dp[0] = 0;
    for i in 1..=n {
        for j in (i as i32 - 100).max(0) as usize..=i {
            if ranges[j] + j as i32 >= i as i32 {
                dp[i] = dp[i].min(dp[(j as i32 - ranges[j]).max(0) as usize] + 1);
            }
        }
    }
    if dp[n] >= i32::MAX / 2 { -1 } else { dp[n] }
}

/// 排序+贪心：每个点能覆盖的区间，贪心找这次能到达最远范围
pub fn min_taps2(n: i32, ranges: Vec<i32>) -> i32 {
    let mut help: Vec<(i32, i32)> = ranges.into_iter().enumerate().map(|(i, r)| {
        (i as i32 - r, i as i32 + r)
    }).collect();
    help.sort_unstable();
    let mut result = 0;
    let mut end = 0;
    let mut i = 0;
    while end < n {
        let mut right = end;
        while i < help.len() && help[i].0 <= end {
            right = right.max(help[i].1);
            i += 1;
        }
        if right == end { break; }
        result += 1;
        end = right;
    }
    if end < n { -1 } else { result }
}

fn main() {
    fn test(func: fn(n: i32, ranges: Vec<i32>) -> i32) {
        assert_eq!(func(7, vec![1, 2, 1, 0, 2, 1, 0, 1]), 3);
        assert_eq!(func(5, vec![3, 4, 1, 1, 0, 0]), 1);
        assert_eq!(func(3, vec![0, 0, 0, 0]), -1);
    }
    test(min_taps);
    test(min_taps2);
}
