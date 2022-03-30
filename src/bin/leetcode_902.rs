//! 最大为 N 的数字组合

pub fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32 {
    let n = n.to_string();
    let n = n.as_bytes();
    let d = digits.iter().map(|x| x.as_bytes()[0]).collect::<Vec<u8>>();
    let mut result = 0;
    for i in 1..n.len() {
        result += (d.len() as f64).powi(i as i32) as i32;
    }
    let mut match_count = 0;
    for i in 0..n.len() {
        match d.binary_search(&n[i]) {
            Ok(v) => {
                match_count += 1;
                result += v as i32 * (d.len() as f64).powi((n.len() - i - 1) as i32) as i32;
            }
            Err(v) => {
                if v == 0 { break; }
                result += v as i32 * (d.len() as f64).powi((n.len() - i - 1) as i32) as i32;
                break;
            }
        };
    }
    result + (match_count == n.len()) as i32
}

/// dp[i] 表示小于等于 N 中最后 i 位数的合法数的个数
pub fn at_most_n_given_digit_set_dp(digits: Vec<String>, n: i32) -> i32 {
    let n = n.to_string();
    let n = n.as_bytes();
    let digits = digits.iter().map(|x| x.as_bytes()[0]).collect::<Vec<u8>>();
    let len = n.len();
    let dlen = digits.len();
    let mut dp = vec![0; len + 1];
    dp[0] = 1;
    for k in 1..=len {
        for i in 0..dlen {
            if digits[i] < n[len - k] {
                dp[k] += (dlen as i32).pow((k - 1) as u32);
            } else if digits[i] == n[len - k] {
                dp[k] += dp[k - 1];
            }
        }
    }
    for i in 1..len {
        dp[len] += (dlen as i32).pow(i as u32);
    }
    dp[len]
}


fn main() {
    assert_eq!(at_most_n_given_digit_set_dp(["5", "7", "8"].iter().map(|x| x.to_string()).collect(), 59), 6);
    assert_eq!(at_most_n_given_digit_set_dp(["3", "4", "5", "6"].iter().map(|x| x.to_string()).collect(), 64), 18);
    assert_eq!(at_most_n_given_digit_set_dp(["3", "4", "8"].iter().map(|x| x.to_string()).collect(), 4), 2);
    assert_eq!(at_most_n_given_digit_set_dp(["7"].iter().map(|x| x.to_string()).collect(), 8), 1);
    assert_eq!(at_most_n_given_digit_set_dp(["7"].iter().map(|x| x.to_string()).collect(), 8), 1);
    assert_eq!(at_most_n_given_digit_set_dp(["1", "3", "5", "7"].iter().map(|x| x.to_string()).collect(), 100), 20);
    assert_eq!(at_most_n_given_digit_set_dp(["1", "4", "9"].iter().map(|x| x.to_string()).collect(), 1000000000), 29523);
}
