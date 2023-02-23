//! 无平方子集计数


/// 背包dp做法：把mask装到一个能容纳mask的背包中。（枚举nums）
pub fn square_free_subsets(nums: Vec<i32>) -> i32 {
    const MOD: i32 = 1e9 as i32 + 7;
    let prime = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    let mut nsq = [None; 31];
    for i in 1..=30 {
        let mut bit = 0;
        let mut num = i as i32;
        for (i, &p) in prime.iter().enumerate() {
            if num % p == 0 {
                bit |= 1 << i;
                num /= p;
            }
        }
        if num == 1 { nsq[i] = Some(bit); }
    }
    let mut dp = vec![0; 1 << prime.len()];
    dp[0] = 1;
    for num in nums {
        if let Some(bit) = nsq[num as usize] {
            for j in (bit..1 << prime.len()).rev() {
                if bit | j == j {
                    dp[j] += dp[bit ^ j];
                    if dp[j] >= MOD { dp[j] -= MOD; }
                }
            }
        }
    }
    (dp.into_iter().fold(0, |acc, x| (acc + x) % MOD) + MOD - 1) % MOD
}

/// 状态压缩：枚举每个bit 与 和bit不相交的其他集合 other  （枚举nsq）
pub fn square_free_subsets2(nums: Vec<i32>) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;
    let prime = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    let mut nsq = [None; 31];
    for i in 1..=30 {
        let mut bit = 0;
        let mut num = i as i32;
        for (i, &p) in prime.iter().enumerate() {
            if num % p == 0 {
                bit |= 1 << i;
                num /= p;
            }
        }
        if num == 1 { nsq[i] = Some(bit); }
    }
    let mut counter = vec![0; 31];
    let mut p = 1;
    for num in nums {
        counter[num as usize] += 1;
        if num == 1 {
            p = (p * 2) % MOD;
        }
    }
    let mask = (1 << prime.len()) - 1;
    let mut dp = vec![0; 1 << prime.len()];  // 空间优化了
    dp[0] = 1;
    for (num, &cnt) in counter.iter().enumerate() {
        if num == 1 { continue; } // 1后面处理
        if let Some(bit) = nsq[num] {
            let other = mask ^ bit;  // other是和bit不相交的，枚举other的子集。
            // 枚举 非空子集模板：
            let mut s = other;
            while s > 0 {
                dp[s | bit] = (dp[s | bit] + dp[s] * cnt) % MOD;
                s = (s - 1) & other;
            }
            dp[bit] = (dp[bit] + dp[0] * cnt) % MOD;  // 空集
        }
    }
    ((dp.into_iter().fold(0, |acc, x| (acc + x) % MOD) * p + MOD - 1) % MOD) as i32
}


fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 1]), 3);
        assert_eq!(func(vec![3, 4, 4, 5]), 3);
        assert_eq!(func(vec![1]), 1);
    }
    test(square_free_subsets);
    test(square_free_subsets2);
}
