//! 好分区的数目

/// 题目是 >= k，一眼背包问题，但是背包是针对 <k的。所以反向求 第一个组/第二个组 元素和<k的方案数
pub fn count_partitions(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    if nums.iter().map(|x| *x as usize).sum::<usize>() < k * 2 { return 0; }
    const MOD: usize = 1e9 as usize + 7;
    let mut dp = vec![0; k];
    dp[0] = 1;  // 一个数都不选也是一个方案

    let mut result = 1;
    // 0-1背包的空间优化写法
    // 前i个num，组成j的 方案有多少种
    for num in nums {
        result = (result * 2) % MOD;
        for j in (num as usize..k).rev() {
            dp[j] = (dp[j] + dp[j - num as usize]) % MOD;
        }
    }
    for num in dp {
        result = (result + MOD + MOD - num * 2) % MOD;
    }
    result as i32
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![1, 2, 3, 4], 4), 6);
        assert_eq!(func(vec![3, 3, 3], 4), 0);
        assert_eq!(func(vec![6, 6], 2), 2);
    }
    test(count_partitions);
}
