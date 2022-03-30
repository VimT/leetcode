//! 最长重复子数组

use std::collections::HashSet;

pub fn find_length(a: Vec<i32>, b: Vec<i32>) -> i32 {
    let a_len = a.len();
    let b_len = b.len();
    let mut dp = vec![vec![0; b_len + 1]; a_len + 1];
    let mut ans = 0;
    for i in 1..=a_len {
        for j in 1..=b_len {
            dp[i][j] = if a[i - 1] == b[j - 1] {
                dp[i - 1][j - 1] + 1
            } else {
                0
            };
            ans = dp[i][j].max(ans);
        }
    }
    ans
}

/// 滑动窗口
pub fn find_length_window(a: Vec<i32>, b: Vec<i32>) -> i32 {
    let a_len = a.len();
    let b_len = b.len();
    let mut ans = 0;
    let max_len = |add_a: usize, add_b: usize, len: usize| -> i32 {
        let mut ans = 0;
        let mut k = 0;
        for i in 0..len {
            k = if a[add_a + i] == b[add_b + i] {
                k + 1
            } else { 0 };
            ans = ans.max(k);
        }
        ans
    };
    for i in 0..a_len {
        let len = b_len.min(a_len - i);
        let maxlen = max_len(i, 0, len);
        ans = ans.max(maxlen);
    }
    for i in 0..b_len {
        let len = a_len.min(b_len - i);
        let maxlen = max_len(0, i, len);
        ans = ans.max(maxlen);
    }
    ans
}

const MOD: u64 = 1e9 as u64 + 9;
const BASE: u64 = 113;

/// 二分查找+hash
/// Rabin-Karp 算法做区间哈希。O(n) 时间内计算Hash(0-n)，O(1) 时间hash滚动
pub fn find_length_bin_search(a: Vec<i32>, b: Vec<i32>) -> i32 {
    let a_len = a.len();
    let b_len = b.len();
    fn quick_pow(mut x: u64, mut n: u64) -> u64 {
        let mut ans = 1;
        while n != 0 {
            if n & 1 == 1 {
                ans = ans * x % MOD;
            }
            x = x * x % MOD;
            n >>= 1;
        }
        ans
    }

    let check = |len: usize| -> bool {
        let mut hash_a = 0;
        for i in 0..len {
            hash_a = (hash_a * BASE + a[i] as u64) % MOD;
        }
        let mult = quick_pow(BASE, (len - 1) as u64);
        let mut set = HashSet::new();
        set.insert(hash_a);
        for i in len..a_len {
            hash_a = ((hash_a - a[i - len] as u64 * mult % MOD + MOD) % MOD * BASE + a[i] as u64) % MOD;
            set.insert(hash_a);
        }
        let mut hash_b = 0;
        for i in 0..len {
            hash_b = (hash_b * BASE + b[i] as u64) % MOD;
        }
        if set.contains(&hash_b) { return true; }
        for i in len..b_len {
            hash_b = ((hash_b - b[i - len] as u64 * mult % MOD + MOD) % MOD * BASE + b[i] as u64) % MOD;
            if set.contains(&hash_b) { return true; }
        }
        false
    };

    let mut left = 1;
    let mut right = a_len.min(b_len) + 1;
    while left < right {
        let mid = left + (right - left) / 2;
        if check(mid) {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    (left - 1) as i32
}


fn main() {
    assert_eq!(find_length_bin_search(vec![1, 2, 3, 2, 1], vec![3, 2, 1, 4, 7]), 3);
}