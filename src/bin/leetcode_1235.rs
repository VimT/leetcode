//! 规划兼职工作

use std::collections::HashMap;

/// 对时间离散化
/// dp[i] 表示 0-i时间内完成的最大profit，dp[i] = max(dp[i-1], dp[j] + profit)
pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
    let len = start_time.len() << 1;
    let mut ts = Vec::with_capacity(len);
    ts.extend_from_slice(&start_time);
    ts.extend_from_slice(&end_time);
    ts.sort_unstable();
    let mut m = HashMap::with_capacity(len);
    for (i, &t) in ts.iter().enumerate() {
        m.insert(t, i);
    }
    let start_time: Vec<usize> = start_time.into_iter().map(|x| m[&x]).collect();
    let end_time: Vec<usize> = end_time.into_iter().map(|x| m[&x]).collect();
    let mut em: Vec<Vec<usize>> = vec![vec![]; len];
    for (i, et) in end_time.into_iter().enumerate() {
        em[et].push(i);
    }
    let mut dp = vec![0; len];
    for i in 1..len {
        dp[i] = dp[i - 1];
        for &j in &em[i] {
            dp[i] = dp[i].max(dp[start_time[j]] + profit[j]);
        }
    }
    dp[len - 1]
}

/// dp[i] 表示前i个工作完成的能完成的最大profit
/// 对 end_time 排序，dp[i] = max(dp[i-1] (不做这个任务), dp[j] + profit[j] (做这个任务))
/// 其中，找上个任务的时间可以用二分查找
pub fn job_scheduling2(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
    let len = profit.len();
    let mut jobs = start_time
        .into_iter()
        .zip(end_time.into_iter())
        .zip(profit.into_iter())
        .map(|((a, b), c)| (b, a, c))
        .collect::<Vec<(i32, i32, i32)>>();
    jobs.sort_unstable();

    let mut dp = vec![0; len + 1];
    for (i, &(_, st, p)) in jobs.iter().enumerate() {
        let j = jobs.binary_search(&(st, i32::MAX, 0)).unwrap_or_else(|x| x);
        dp[i + 1] = dp[i].max(dp[j] + p);
    }

    dp[len]
}

fn main() {
    fn test(func: fn(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 2, 3, 3], vec![3, 4, 5, 6], vec![50, 10, 40, 70]), 120);
        assert_eq!(func(vec![1, 2, 3, 4, 6], vec![3, 5, 10, 6, 9], vec![20, 20, 100, 70, 60]), 150);
        assert_eq!(func(vec![1, 1, 1], vec![2, 3, 4], vec![5, 6, 4]), 6);
    }
    test(job_scheduling);
    test(job_scheduling2);
}
