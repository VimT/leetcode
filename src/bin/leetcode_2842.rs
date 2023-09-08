//! 统计一个字符串的 k 子序列美丽值最大的数目

use std::collections::HashMap;

fn pow(n: i64, k: i64, mod_: i64) -> i64 {
    let mut result = 1;
    for _ in 0..k { result = result * n % mod_; }
    result
}

fn comb(n: i64, k: i64) -> i64 {
    let mut result = n;
    for i in 1..k {
        result = result * (n - i) / (i + 1);
    }
    result
}

pub fn count_k_subsequences_with_max_beauty(s: String, k: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let s = s.as_bytes();
    let mut cnt = [0; 26];
    for &ch in s {
        cnt[(ch - b'a') as usize] += 1;
    }
    let mut cc: HashMap<i64, i64> = HashMap::new();
    for num in cnt {
        if num > 0 { *cc.entry(num).or_default() += 1; }
    }
    let mut cc: Vec<(i64, i64)> = cc.into_iter().collect();
    cc.sort_unstable_by_key(|x| -x.0);
    let mut result = 1;
    let mut k = k as i64;
    for (c, num) in cc {
        if num >= k {
            return (result * pow(c, k, MOD) % MOD * comb(num, k) % MOD) as i32;
        }
        result = result * pow(c, num, MOD) % MOD;
        k -= num;
    }
    0
}

fn main() {
    fn test(func: fn(s: String, k: i32) -> i32) {
        assert_eq!(func(String::from("cninkeiqk"), 4), 24);
        assert_eq!(func(String::from("dd"), 2), 0);
        assert_eq!(func(String::from("bcca"), 2), 4);
        assert_eq!(func(String::from("abbcd"), 4), 2);
    }
    test(count_k_subsequences_with_max_beauty);
}
