//! 求出数组中最大序列值

pub fn max_value(nums: Vec<i32>, k: i32) -> i32 {
    let len = nums.len();
    let k = k as usize;
    let mx = nums.iter().fold(0, |acc, &x| acc | x) as usize;
    let mut suf = vec![vec![false; mx + 1]; len];
    let mut dp = vec![vec![false; mx + 1]; k + 1]; // dp[i][j][x] 表示前i个数选j个数，异或和能否为x （第一维省略）
    dp[0][0] = true;
    for i in (k - 1..len).rev() {
        let v = nums[i] as usize;
        for j in (0..k).rev() {
            for x in 0..=mx {
                if dp[j][x] {
                    dp[j + 1][x | v] = true;
                }
            }
        }
        suf[i] = dp[k].clone();
    }
    let mut result = 0;
    dp = vec![vec![false; mx + 1]; k + 1];
    dp[0][0] = true;
    for (i, &v) in nums[..len - k].iter().enumerate() {
        for j in (0..k).rev() {
            for x in 0..=mx {
                if dp[j][x] {
                    dp[j + 1][x | v as usize] = true;
                }
            }
        }
        if i >= k - 1 {
            for (x, &has_x) in dp[k].iter().enumerate() {
                if has_x {
                    for (y, &has_y) in suf[i + 1].iter().enumerate() {
                        if has_y {
                            result = result.max(x ^ y);
                        }
                    }
                }
            }
        }
    }

    result as i32
}

/// 用 u128 优化空间（速度其实更慢）
pub fn max_value2(nums: Vec<i32>, k: i32) -> i32 {
    let len = nums.len();
    let k = k as usize;
    let mx = nums.iter().fold(0, |acc, &x| acc | x) as usize;
    let mut suf = vec![1u128; len];
    let mut dp = vec![0u128; k + 1];
    dp[0] = 1;
    for i in (k - 1..len).rev() {
        let v = nums[i] as usize;
        for j in (0..k).rev() {
            for x in 0..=mx {
                if dp[j] >> x & 1 == 1 {
                    dp[j + 1] |= 1 << (x | v);
                }
            }
        }
        suf[i] = dp[k];
    }
    let mut result = 0;
    dp = vec![0; k + 1];
    dp[0] = 1;
    for (i, &v) in nums[..len - k].iter().enumerate() {
        for j in (0..k).rev() {
            for x in 0..=mx {
                if dp[j] >> x & 1 == 1 {
                    dp[j + 1] |= 1 << (x | v as usize);
                }
            }
        }
        if i >= k - 1 {
            for x in 0..=mx {
                if dp[k] >> x & 1 == 1 {
                    for y in 0..=mx {
                        if suf[i + 1] >> y & 1 == 1 {
                            result = result.max(x ^ y);
                        }
                    }
                }
            }
        }
    }

    result as i32
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![2, 6, 7], 1), 5);
        assert_eq!(func(vec![4, 2, 5, 6, 7], 2), 2);
    }
    test(max_value);
    test(max_value2);
}
