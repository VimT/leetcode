//! 和带限制的子多重集合的数目

use std::collections::HashMap;

/// 普通DP，超时
pub fn count_sub_multisets(nums: Vec<i32>, l: i32, r: i32) -> i32 {
    const MOD: i32 = 1e9 as i32 + 7;
    let mut cnt_map: HashMap<usize, usize> = HashMap::new();
    let mut sum = 0;
    for num in nums {
        *cnt_map.entry(num as usize).or_default() += 1;
        sum += num as usize;
    }
    let mut dp = vec![0; sum + 1];  // dp[i] 表示和为i时，集合的数目
    dp[0] = cnt_map.remove(&0).map(|x| x as i32).unwrap_or(0) + 1;
    for (num, cnt) in cnt_map {
        for s in (num..=sum).rev() {
            for j in 1..=cnt {
                if s < j * num { break; }
                dp[s] = (dp[s] + dp[s - j * num]) % MOD;
            }
        }
    }

    dp[l as usize..=r as usize].iter().fold(0, |a, &b| (a + b) % MOD)
}


/// 减少重复计算
/// 仔细观察 dp[i][j] 和 dp[i][j-x]，可以得到 dp[i][j] = dp[i-1][j] + dp[i][j-num] - dp[i-1][j-(cnt+1)*num]
pub fn count_sub_multisets2(nums: Vec<i32>, l: i32, r: i32) -> i32 {
    const MOD: i32 = 1e9 as i32 + 7;
    let mut cnt_map: HashMap<usize, usize> = HashMap::new();
    let mut sum = 0;
    for num in nums {
        *cnt_map.entry(num as usize).or_default() += 1;
        sum += num as usize;
    }
    if sum < l as usize { return 0; }
    sum = sum.min(r as usize);
    let mut dp = vec![0; sum + 1];  // dp[i] 表示和为i时，集合的数目
    dp[0] = cnt_map.remove(&0).map(|x| x as i32).unwrap_or(0) + 1;
    let mut up = 0;
    for (num, cnt) in cnt_map {
        let mut dp2 = dp.clone();
        up = sum.min(up + cnt * num);
        for s in num..=up {
            dp2[s] = (dp2[s] + dp2[s - num]) % MOD;
            if s >= (cnt + 1) * num {
                dp2[s] -= dp[s - (cnt + 1) * num];
                dp2[s] += MOD;
            }
            dp2[s] %= MOD;
        }
        dp = dp2;
    }
    dp[l as usize..=sum].iter().fold(0, |a, &b| (a + b) % MOD)
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, l: i32, r: i32) -> i32) {
        assert_eq!(func(vec![23, 54, 2, 21, 43, 41, 5, 9, 27, 6, 41, 27, 18, 20, 9, 12, 8, 9, 57, 13, 31, 25, 33, 11, 30, 12, 34, 19, 1, 12, 13, 40, 28, 40, 22, 4, 36, 8, 11, 5, 9, 11, 34, 13, 20, 20, 25, 14, 9, 19, 89, 5, 37, 4, 6, 32, 44, 1, 2, 28, 6, 15, 26, 9, 60, 2, 9, 4, 11, 36, 63, 18, 6, 79, 6, 1, 8, 37, 22, 15, 16, 0, 15, 1, 54, 6, 11, 11, 4, 5, 36, 27, 17, 33, 30, 19], 122, 474), 391827978);
        assert_eq!(func(vec![0, 0, 1, 2, 3], 2, 3), 9);
        assert_eq!(func(vec![1, 2, 2, 3], 6, 6), 1);
        assert_eq!(func(vec![2, 1, 4, 2, 7], 1, 5), 7);
        assert_eq!(func(vec![1, 2, 1, 3, 5, 2], 3, 5), 9);
    }
    test(count_sub_multisets);
    test(count_sub_multisets2);
}
