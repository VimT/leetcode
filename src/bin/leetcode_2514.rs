//! 统计同位异构字符串数目

use leetcode::algorithm::quick_pow;

/// 有重复的排列数：有m个元素，值为v1的个数有n1个...，则排列数 = (m!) / (n1! * n2! * ... * nn! )
/// 乘法逆元：a / b (mod P) = a * b^-1 (mod P) = a * pow(b, P-2, P)
pub fn count_anagrams(s: String) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;
    let mut a = 1;
    let mut b = 1;
    for word in s.split(' ') {
        let mut cnt = [0; 26];
        for (i, &ch) in word.as_bytes().iter().enumerate() {
            a = a * (i + 1) as i64 % MOD;
            cnt[(ch - b'a') as usize] += 1;
            b = b * cnt[(ch - b'a') as usize] % MOD;
        }
    }
    (a * quick_pow(b, MOD - 2, MOD) % MOD) as i32
}

fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("rsrybprxlendseni")), 891452955);
        assert_eq!(func(String::from("too hot")), 18);
        assert_eq!(func(String::from("aa")), 1);
    }
    test(count_anagrams);
}
