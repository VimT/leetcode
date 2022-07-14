//! 统计打字方案数

pub fn count_texts(pressed_keys: String) -> i32 {
    const MOD: i32 = 1e9 as i32 + 7;
    let s = pressed_keys.as_bytes();
    let len = s.len();
    let mut dp = vec![0; len + 1];
    dp[0] = 1;
    dp[1] = 1;
    let mut same_cnt = 1;
    for i in 2..=len {
        if s[i - 1] != s[i - 2] {
            same_cnt = 1;
        } else {
            same_cnt += 1;
        }
        let min = if matches!(s[i-1], b'7'|b'9') { 4 } else { 3 };
        for j in i - same_cnt.min(min)..i {
            dp[i] = (dp[i] + dp[j]) % MOD;
        }
    }
    dp[len]
}

fn main() {
    assert_eq!(count_texts(String::from("22233")), 8);
    assert_eq!(count_texts(String::from("222222222222222222222222222222222222")), 82876089);
}
