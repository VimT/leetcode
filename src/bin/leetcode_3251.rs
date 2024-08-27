//! 单调数组对的数目 II

/// 原版：
/// for i in (0..len - 1).rev() {
///     for ai in 0..=nums[i] {
///         let bi = nums[i] - ai;
///         for ai1 in ai..=nums[i + 1] {
///             bi1 = nums[i + 1] - ai1;
///             if bi1 <= bi {
///                dp[i][ai] = (dp[i][ai] + dp[i + 1][ai1]) % MOD;
///  使用前缀和优化
pub fn count_of_pairs(nums: Vec<i32>) -> i32 {
    const MOD: i32 = 1e9 as i32 + 7;
    let len = nums.len();
    let mx = nums.iter().max().copied().unwrap() as usize;
    let mut dp = vec![vec![0; mx + 1]; len]; // dp[i][j] 表示在第 i 个数字时，arr1[i]=j 的方案数
    dp[len - 1][..=nums[len - 1] as usize].fill(1);
    let mut presum = vec![0; mx + 2];
    for i in (0..len - 1).rev() {
        for j in 0..=mx { presum[j + 1] = (presum[j] + dp[i + 1][j]) % MOD; }
        for ai in 0..=nums[i] {
            let bi = nums[i] - ai;
            dp[i][ai as usize] = (presum[nums[i + 1] as usize + 1] - presum[ai.max(nums[i + 1] - bi) as usize] + MOD) % MOD;
        }
    }
    dp[0][0..=nums[0] as usize].into_iter().fold(0, |a, &b| (a + b) % MOD)
}

/// 空间优化
pub fn count_of_pairs2(nums: Vec<i32>) -> i32 {
    const MOD: i32 = 1e9 as i32 + 7;
    let len = nums.len();
    let m = nums[len - 1] as usize;
    let mut f = vec![0; m + 1];
    f[..=m.min(nums[0] as usize)].fill(1);
    for win in nums.windows(2) {
        let j0 = (win[1] - win[0]).max(0) as usize;
        let m2 = (win[1] as usize).min(m);
        if j0 > m2 { return 0; }
        for j in 1..=m2 - j0 {
            f[j] = (f[j] + f[j - 1]) % MOD;  // 前缀和
        }
        f.copy_within(0..=m2 - j0, j0);
        f[..j0].fill(0);
    }
    f.into_iter().fold(0, |a, b| (a + b) % MOD)
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![40, 91, 187, 256, 334, 337, 694, 749, 782, 833]), 272278100);
        assert_eq!(func(vec![2, 3, 2]), 4);
        assert_eq!(func(vec![5, 5, 5, 5]), 126);
    }
    test(count_of_pairs);
    test(count_of_pairs2);
}
