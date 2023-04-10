//! 统计字典序元音字符串的数目

pub fn count_vowel_strings(n: i32) -> i32 {
    let mut dp = vec![1; 5]; // dp[i]表示前i位数，以j开头的字典序排列有多少种
    for _ in 1..n as usize {
        let mut new_dp = vec![0; 5];
        for j in 0..5 {
            for k in 0..=j {
                new_dp[j] += dp[k];
            }
        }
        dp = new_dp;
    }
    dp.iter().sum()
}

pub fn count_vowel_strings2(n: i32) -> i32 {
    (n + 1) * (n + 2) * (n + 3) * (n + 4) / 24
}

fn main() {
    fn test(func: fn(n: i32) -> i32) {
        assert_eq!(func(1), 5);
        assert_eq!(func(2), 15);
        assert_eq!(func(33), 66045);
    }
    test(count_vowel_strings);
    test(count_vowel_strings2);
}
