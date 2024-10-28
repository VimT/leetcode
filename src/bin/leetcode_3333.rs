//! 找到初始输入字符串 II

/// 正难则反
pub fn possible_string_count(word: String, k: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let k = k as usize;
    if word.len() < k { return 0; }
    let mut groups = vec![];
    let mut i = 0;
    let s = word.as_bytes();
    let len = s.len();
    let mut result = 1;
    while i < len {
        let start = i;
        while i < len && s[i] == s[start] {
            i += 1;
        }
        result = result * (i - start) as i64 % MOD;
        groups.push(i - start);
    }
    if groups.len() >= k {
        return result as i32;
    }
    let len = groups.len();
    let mut dp = vec![vec![0; k]; len + 1]; // dp[i][j] 表示用前 i 组构造长为 j 的方案数
    dp[0][0] = 1;
    for (i, c) in groups.into_iter().enumerate() {
        let mut pre_sum = vec![0; k + 1];
        for j in 0..k {
            pre_sum[j + 1] = (pre_sum[j] + dp[i][j]) % MOD;
        }
        // j <= i 的 dp[i][j]  都是 0
        for j in i + 1..k {
            dp[i + 1][j] = (pre_sum[j] - pre_sum[if j >= c { j - c } else { 0 }]).rem_euclid(MOD);
        }
    }
    (result - dp[len][len..].iter().fold(0, |acc, &num| (acc + num) % MOD)).rem_euclid(MOD) as i32
}

fn main() {
    fn test(func: fn(word: String, k: i32) -> i32) {
        assert_eq!(func(String::from("d"), 1), 1);
        assert_eq!(func(String::from("aabbccdd"), 7), 5);
        assert_eq!(func(String::from("aabbccdd"), 8), 1);
        assert_eq!(func(String::from("aaabbb"), 3), 8);
    }
    test(possible_string_count);
}
