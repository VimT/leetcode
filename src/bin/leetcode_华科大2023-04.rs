//! 美丽字符串

use leetcode::algorithm::quick_pow;

pub fn beautiful_string(s: String) -> i32 {
    const MOD: i64 = 998244353;
    const MAX_N: usize = 2e5 as usize + 2;
    // 逆元求组合数
    let c = unsafe {
        static mut FAC: [i64; MAX_N] = [1; MAX_N];
        static mut FACINV: [i64; MAX_N] = [1; MAX_N];
        if FAC[2] == 1 {
            for i in 1..MAX_N {
                FAC[i] = FAC[i - 1] * i as i64 % MOD;
            }
            FACINV[MAX_N - 1] = quick_pow(FAC[MAX_N - 1], MOD - 2, MOD);
            for i in (1..MAX_N).rev() {
                FACINV[i - 1] = FACINV[i] * i as i64 % MOD;
            }
        }
        |n: i64, m: i64| -> i64 {
            if m < 0 || n < m { return 0; }
            FAC[n as usize] * FACINV[m as usize] % MOD * FACINV[(n - m) as usize] % MOD
        }
    };
    let s = s.as_bytes();
    let len = s.len();
    let mut dp = vec![[0, 0]; len]; // dp[i][type] 表示 s 的前i位，满足type类型的方案数
    let mut one_cnt = (s[0] - b'0') as i64;
    let mut result = 1;
    dp[0][(s[0] - b'0') as usize] = 1;
    let mut total = 2;
    for (i, &ch) in s.iter().enumerate().skip(1) {
        let ch = (ch - b'0') as usize;
        let cnt = if ch == 1 { one_cnt } else { i as i64 - one_cnt };
        let before = i as i64 - 1;
        dp[i][ch] = (dp[i - 1][ch] * 2 + c(before, before - cnt)) % MOD;  // 都可以填 + 只可以填 ch
        dp[i][ch ^ 1] = (total + MOD - dp[i][ch]) % MOD;

        result = (result + dp[i][ch]) % MOD;
        one_cnt += ch as i64;
        total = (total * 2) % MOD;
    }
    result as i32
}

fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("11")), 3);
        assert_eq!(func(String::from("01001")), 17);
        assert_eq!(func(String::from("00011")), 13);
    }
    test(beautiful_string);
}
