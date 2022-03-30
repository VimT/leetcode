//! 解码方法

/// dp[i] i->结尾
pub fn num_decodings(s: String) -> i32 {
    let mut n: Vec<u8> = s.as_bytes().iter().map(|x| *x - b'0').collect();
    let mut m = vec![];
    let mut i = 0;
    while i < n.len() {
        let mut num = n[i];
        if i < n.len() - 1 && n[i + 1] == 0 {
            num = num * 10;
            i += 2;
        } else {
            i += 1;
        }
        if num < 1 || num > 26 { return 0; }

        m.push(num);
    }
    let len = m.len();
    if len == 0 { return 0; }
    let mut dp = vec![1; len + 1];
    for i in 2..=len {
        dp[i] = if m[len - i] >= 10 || m[len - i + 1] >= 10 || m[len - i] * 10 + m[len - i + 1] > 26 {
            dp[i - 1]
        } else {
            dp[i - 1] + dp[i - 2]
        }
    }

    dp[len]
}

fn main() {
    assert_eq!(num_decodings("110".to_string()), 1);
    assert_eq!(num_decodings("100".to_string()), 0);
    assert_eq!(num_decodings("00".to_string()), 0);
    assert_eq!(num_decodings("0".to_string()), 0);
    assert_eq!(num_decodings("10".to_string()), 1);
    assert_eq!(num_decodings("101".to_string()), 1);
    assert_eq!(num_decodings("1101".to_string()), 1);
    assert_eq!(num_decodings("101101".to_string()), 1);
    assert_eq!(num_decodings("01".to_string()), 0);
    assert_eq!(num_decodings("226".to_string()), 3);
    assert_eq!(num_decodings("12".to_string()), 2);
}

