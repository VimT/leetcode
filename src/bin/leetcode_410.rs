//! 分割数组的最大值


/// 「使……最大值尽可能小」是二分搜索题目常见的问法。
pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
    let nums = nums.into_iter().map(|x| x as i64).collect::<Vec<i64>>();
    let mut left = *nums.iter().max().unwrap_or(&0);
    let mut right = nums.iter().sum::<i64>();
    let check = |value: i64| {
        let mut cnt = 0;
        let mut current = 0;
        for i in &nums {
            if current + *i > value {
                cnt += 1;
                current = 0;
            }
            current += *i;
        }
        if current != 0 { cnt += 1; }
        cnt
    };
    while left < right {
        let mid = left + (right - left) / 2;
        if check(mid) <= m {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left as i32
}

/// 「将数组分割为 mm 段，求……」是动态规划题目常见的问法。
/// 本题中，我们可以令 f[i][j] 表示将数组的前 i 个数分割为 j 段所能得到的最大连续子数组和的最小值
/// 枚举 k，其中前 k 个数被分割为 j-1 段，而第 k+1 到第 i 个数为第 j 段
pub fn split_array_dp(nums: Vec<i32>, m: i32) -> i32 {
    let len = nums.len();
    let m = m as usize;
    let mut dp = vec![vec![i64::max_value(); m + 1]; len + 1];
    let mut sub = vec![0; len + 1];
    for i in 0..len {
        sub[i + 1] = sub[i] + nums[i] as i64;
    }
    dp[0][0] = 0;
    for i in 1..=len {
        for j in 1..=i.min(m) {
            for k in 0..i {
                dp[i][j] = dp[i][j].min(dp[k][j - 1].max(sub[i] - sub[k]));
            }
        }
    }

    dp[len][m] as i32
}


fn main() {
    fn test(func: fn(nums: Vec<i32>, m: i32) -> i32) {
        assert_eq!(func(vec![7, 2, 5, 10, 8], 2), 18);
        assert_eq!(func(vec![1, 2, 3, 4, 5], 2), 9);
        assert_eq!(func(vec![1, 4, 4], 3), 4);
    }
    test(split_array);
    test(split_array_dp);
}
